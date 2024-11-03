// TODO Things to optimize:
// - agents dont always have to be freed, in many rules they can be
//   repurposed, saving the cost of moving an agent.
//   The rules where agents to be deleted: L-E, S-E, L-A, S-A, F-A, L-T, F-T
// - The current memory handling works, but might not be the fastest, and for
//   embedded devices, it is wasteful
// - Need to make it parallelizable (look at how HVM solved evaluating subnets
//   that connect to each other)

#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

// #define DEBUG_PRINTS

#ifdef DEBUG_PRINTS
#include <stdarg.h>
#endif

#define AGENT_L    0
#define AGENT_S  257
#define AGENT_F  514
#define AGENT_E  768
#define AGENT_D 1026
#define AGENT_A 1282
#define AGENT_T 1539
#define AGENT_Q 1796
#define AGENT_I 2049

char* agent_type_str = "LSFEDATQI";

#define P0 0
#define P1 1
#define P2 2
#define P3 3
#define PMAIN 4

#define AGENT_BLOCK_SIZE 4096
#define PAIR_BLOCK_SIZE 128

enum ConnectType {NO_REF, SRC_REF, DST_REF, FULL_REF};

struct Agent {
    uint8_t type;
    uint8_t port_count;
    uint8_t port_numbers[5];
    // These pointers allow this agent to be moved - when its address changes,
    // the new address should be put at the places these addresses point to
    struct Agent** pair_stack_addr;
    struct Agent* ports[5]; // ports[4] is the agent connected to the main port
};

struct Pair {
    struct Agent* agent0;
    struct Agent* agent1;
};

struct AgentBlock {
    struct AgentBlock* prev;
    struct AgentBlock* next;
    struct Agent* end;
    struct Agent data[AGENT_BLOCK_SIZE];
};

struct AgentStack {
    struct Agent* next_free_addr;
    struct AgentBlock* current_block;
};

struct PairBlock {
    struct PairBlock* prev;
    struct PairBlock* next;
    struct Pair* end;
    struct Pair data[AGENT_BLOCK_SIZE];
};

struct PairStack {
    struct Pair* next_free_addr;
    struct PairBlock* current_block;
};

typedef void (*Rule)(struct Agent*, struct Agent*);

// Rule forward decls
void rule_l_e(struct Agent*, struct Agent*);
void rule_l_d(struct Agent*, struct Agent*);
void rule_l_a(struct Agent*, struct Agent*);
void rule_l_t(struct Agent*, struct Agent*);
void rule_l_q(struct Agent*, struct Agent*);
void rule_s_e(struct Agent*, struct Agent*);
void rule_s_d(struct Agent*, struct Agent*);
void rule_s_a(struct Agent*, struct Agent*);
void rule_s_t(struct Agent*, struct Agent*);
void rule_s_q(struct Agent*, struct Agent*);
void rule_f_e(struct Agent*, struct Agent*);
void rule_f_d(struct Agent*, struct Agent*);
void rule_f_a(struct Agent*, struct Agent*);
void rule_f_t(struct Agent*, struct Agent*);
void rule_f_q(struct Agent*, struct Agent*);

void debug_print(int);
void debug(const char* s, ...) {
    #ifdef DEBUG_PRINTS
    va_list args;
    va_start(args, s);
    vprintf(s, args);
    va_end(args);
    #endif
}

// GLOBALS --------
static struct AgentStack agent_stack;
static struct PairStack pair_stack;
static const Rule rule_table[15] = {
    rule_l_e, rule_l_d, rule_l_a, rule_l_t, rule_l_q,
    rule_s_e, rule_s_d, rule_s_a, rule_s_t, rule_s_q,
    rule_f_e, rule_f_d, rule_f_a, rule_f_t, rule_f_q
};
// -------- GLOBALS

// Initialize globals
void init_globals() {
    // agent_stack.data = malloc(AGENT_STACK_SIZE * sizeof(struct Agent));
    struct AgentBlock* agent_block = malloc(sizeof(struct AgentBlock));
    agent_block->next = NULL;
    agent_block->prev = NULL;
    agent_block->end = agent_block->data + AGENT_BLOCK_SIZE;
    agent_stack.next_free_addr = agent_block->data;
    agent_stack.current_block = agent_block;

    // pair_stack.data = malloc(PAIR_STACK_SIZE * sizeof(struct Pair));
    struct PairBlock* pair_block = malloc(sizeof(struct PairBlock));
    pair_block->next = NULL;
    pair_block->prev = NULL;
    pair_block->end = pair_block->data + PAIR_BLOCK_SIZE;
    pair_stack.next_free_addr = pair_block->data;
    pair_stack.current_block = pair_block;
}

// Put an agent on the stack and return the pointer to the agent
struct Agent* store_agent(struct Agent agent) {
    *agent_stack.next_free_addr = agent;
    struct Agent* result = agent_stack.next_free_addr;
    agent_stack.next_free_addr++;
    if (agent_stack.next_free_addr == agent_stack.current_block->end) {
        struct AgentBlock* agent_block = agent_stack.current_block->next;
        if (agent_block == NULL) {
            agent_block = malloc(sizeof(struct AgentBlock));
            agent_stack.current_block->next = agent_block;
            agent_block->prev = agent_stack.current_block;
            agent_block->next = NULL;
            agent_block->end = agent_block->data + AGENT_BLOCK_SIZE;
        }
        agent_stack.next_free_addr = agent_block->data;
        agent_stack.current_block = agent_block;
    }
    return result;
}

// Put a pair on the stack and update the agent's pair pointers
void store_pair(struct Pair pair) {
    *pair_stack.next_free_addr = pair;
    struct Agent* agent0 = pair_stack.next_free_addr->agent0;
    struct Agent* agent1 = pair_stack.next_free_addr->agent1;
    agent0->pair_stack_addr = &pair_stack.next_free_addr->agent0;
    agent1->pair_stack_addr = &pair_stack.next_free_addr->agent1;
    pair_stack.next_free_addr++;
    if (pair_stack.next_free_addr == pair_stack.current_block->end) {
        struct PairBlock* pair_block = pair_stack.current_block->next;
        if (pair_block == NULL) {
            pair_block = malloc(sizeof(struct PairBlock));
            pair_stack.current_block->next = pair_block;
            pair_block->prev = pair_stack.current_block;
            pair_block->next = NULL;
            pair_block->end = pair_block->data + PAIR_BLOCK_SIZE;
        }
        pair_stack.next_free_addr = pair_block->data;
        pair_stack.current_block = pair_block;
    }
}

// Compact the stack, and update pointers
void free_agent(struct Agent* stack_slot) {
    debug("=== Freeing %lu ===\n", (size_t)stack_slot);
    if (agent_stack.next_free_addr == agent_stack.current_block->data) {
        agent_stack.current_block = agent_stack.current_block->prev;
        agent_stack.next_free_addr = agent_stack.current_block->end;
    }
    agent_stack.next_free_addr--;
    if (stack_slot == agent_stack.next_free_addr) {
        return;
    }

    debug("    Copying top of stack\n");
    // Copy the top of the stack to the ptr
    *stack_slot = *agent_stack.next_free_addr;

    // Update the parent's and children's ptr
    if (stack_slot->ports[PMAIN] != NULL)  {
        debug("    Updating parent's ptr %lu\n",
            (size_t)stack_slot->ports[PMAIN]);
        stack_slot->ports[PMAIN]->ports[stack_slot->port_numbers[PMAIN]] =
            stack_slot;
    } else {
        debug(">>> [PANIC] AGENT PARENT IS NULL <<<\n");
    }
    for (int i = 0; i < stack_slot->port_count; i++) {
        if (stack_slot->ports[i] != NULL)  {
            debug("    Updating child's ptr %lu\n",
                (size_t)stack_slot->ports[i]);
            stack_slot->ports[i]->ports[stack_slot->port_numbers[i]] =
                stack_slot;
        } else {
            debug(">>> [PANIC] AGENT CHILD %d / %d IS NULL <<<\n", i,
                stack_slot->port_count);
        }
    }

    debug("    Updating stack ptr %lu\n", (size_t)stack_slot->pair_stack_addr);
    // Update the stack's ptr
    if (stack_slot->pair_stack_addr != NULL) {
        *(stack_slot->pair_stack_addr) = stack_slot;
    }
}

// Pop an equation off the stack, execute the corresponding rule, and free the
// agents afterwards
uint8_t step() {
    debug("Pop pair stack\n");
    if (pair_stack.next_free_addr == pair_stack.current_block->data) {
        if (pair_stack.current_block->prev == NULL) {
            return 1;
        } else {
            pair_stack.current_block = pair_stack.current_block->prev;
            pair_stack.next_free_addr = pair_stack.current_block->end;
        }
    }

    pair_stack.next_free_addr--;
    struct Pair pair = *pair_stack.next_free_addr;

    struct Agent* agent0_addr = pair.agent0;
    struct Agent* agent1_addr = pair.agent1;
    if (agent0_addr->type == 8 || agent1_addr->type == 8) {
        return 1;
    }

    // DEBUG Check if any of the agents refer to a deleted agent
    for (int i = -1; i < agent0_addr->port_count; i++) {
        if (agent0_addr->ports[i%5] > agent_stack.next_free_addr) {
            debug("Agent0 %lu port %d refers to deleted agent %lu\n",
                (size_t)agent0_addr, i, (size_t)agent0_addr->ports[i%5]);
            exit(EXIT_FAILURE);
        }
    }
    for (int i = -1; i < agent1_addr->port_count; i++) {
        if (agent1_addr->ports[i%5] > agent_stack.next_free_addr) {
            debug("Agent1 %lu port %d refers to deleted agent %lu\n",
                (size_t)agent1_addr, i, (size_t)agent1_addr->ports[i%5]);
            exit(EXIT_FAILURE);
        }
    }

    uint8_t rule_index = agent0_addr->type * 5 + agent1_addr->type - 3;
    debug("Execute rule %d\n", rule_index);
    (*rule_table[rule_index])(agent0_addr, agent1_addr);
    debug("Free agents\n");
    free_agent(agent0_addr);
    free_agent(agent1_addr);

    return 0;
}

// RULE COMPONENTS -----
struct Agent* mk_agent(uint16_t agent_type) {
    struct Agent agent;
    agent.type = (uint8_t)(agent_type >> 8);
    agent.port_count = (uint8_t)agent_type;
    agent.pair_stack_addr = NULL;

    return store_agent(agent);
}

void push(struct Agent* agent0, struct Agent* agent1) {
    struct Pair pair = {agent0, agent1};
    store_pair(pair);
}

void connect(struct Agent* agent0, uint8_t port_num0, struct Agent* agent1,
    uint8_t port_num1, enum ConnectType type)
{
    if (type == SRC_REF || type == FULL_REF) {
        struct Agent* new_agent = agent0->ports[port_num0];
        port_num0 = agent0->port_numbers[port_num0];
        agent0 = new_agent;
    }
    if (type == DST_REF || type == FULL_REF) {
        struct Agent* new_agent = agent1->ports[port_num1];
        port_num1 = agent1->port_numbers[port_num1];
        agent1 = new_agent;
    }
    agent0->ports[port_num0] = agent1;
    agent1->ports[port_num1] = agent0;
    agent0->port_numbers[port_num0] = port_num1;
    agent1->port_numbers[port_num1] = port_num0;
    if (port_num0 == PMAIN && port_num1 == PMAIN) {
        push(agent0, agent1);
    }
}

// ----- RULE COMPONENTS

void rule_l_e(struct Agent* left, struct Agent* right) {
    (void)left;
    (void)right;
    return;
}

void rule_l_d(struct Agent* left, struct Agent* right) {
    (void)left;
    struct Agent* agent0 = mk_agent(AGENT_L);
    struct Agent* agent1 = mk_agent(AGENT_L);
    connect(agent0, PMAIN, right, P0, DST_REF);
    connect(agent1, PMAIN, right, P1, DST_REF);
    return;
}

void rule_l_a(struct Agent* left, struct Agent* right) {
    (void)left;
    struct Agent* agent0 = mk_agent(AGENT_S);
    connect(agent0, PMAIN, right, P1, DST_REF);
    connect(agent0, P0, right, P0, DST_REF);
    return;
}

void rule_l_t(struct Agent* left, struct Agent* right) {
    (void)left;
    struct Agent* agent0 = mk_agent(AGENT_E);
    connect(right, P1, agent0, PMAIN, SRC_REF);
    connect(right, P0, right, P2, FULL_REF);
    return;
}

void rule_l_q(struct Agent* left, struct Agent* right) {
    (void)left;
    struct Agent* agent0 = mk_agent(AGENT_E);
    struct Agent* agent1 = mk_agent(AGENT_E);
    connect(right, P0, right, P3, FULL_REF);
    connect(right, P1, agent0, PMAIN, SRC_REF);
    connect(right, P2, agent1, PMAIN, SRC_REF);
    return;
}

void rule_s_e(struct Agent* left, struct Agent* right) {
    (void)right;
    struct Agent* agent0 = mk_agent(AGENT_E);
    connect(left, P0, agent0, PMAIN, SRC_REF);
    return;
}

void rule_s_d(struct Agent* left, struct Agent* right) {
    struct Agent* agent0 = mk_agent(AGENT_D);
    struct Agent* agent1 = mk_agent(AGENT_S);
    struct Agent* agent2 = mk_agent(AGENT_S);
    connect(agent0, P0, agent1, P0, NO_REF);
    connect(agent0, P1, agent2, P0, NO_REF);
    connect(left, P0, agent0, PMAIN, SRC_REF);
    connect(agent1, PMAIN, right, P0, DST_REF);
    connect(agent2, PMAIN, right, P1, DST_REF);
    return;
}

void rule_s_a(struct Agent* left, struct Agent* right) {
    struct Agent* agent0 = mk_agent(AGENT_F);
    connect(agent0, P0, left, P0, DST_REF);
    connect(agent0, P1, right, P0, DST_REF);
    connect(agent0, PMAIN, right, P1, DST_REF);
    return;
}

void rule_s_t(struct Agent* left, struct Agent* right) {
    struct Agent* agent0 = mk_agent(AGENT_A);
    struct Agent* agent1 = mk_agent(AGENT_A);
    struct Agent* agent2 = mk_agent(AGENT_D);
    struct Agent* agent3 = mk_agent(AGENT_A);
    connect(agent0, P0, agent2, P0, NO_REF);
    connect(agent0, P1, agent1, P0, NO_REF);
    connect(agent1, PMAIN, agent3, P1, NO_REF);
    connect(agent2, P1, agent3, P0, NO_REF);
    connect(right, P0, agent0, PMAIN, SRC_REF);
    connect(right, P2, agent1, P1, SRC_REF);
    connect(right, P1, agent2, PMAIN, SRC_REF);
    connect(left, P0, agent3, PMAIN, SRC_REF);
    return;
}

void rule_s_q(struct Agent* left, struct Agent* right) {
    struct Agent* agent0 = mk_agent(AGENT_A);
    struct Agent* agent1 = mk_agent(AGENT_E);
    struct Agent* agent2 = mk_agent(AGENT_E);
    connect(agent0, P0, left, P0, DST_REF);
    connect(agent0, P1, right, P3, DST_REF);
    connect(right, P1, agent0, PMAIN, SRC_REF);
    connect(right, P0, agent1, PMAIN, SRC_REF);
    connect(right, P2, agent2, PMAIN, SRC_REF);
    return;
}

void rule_f_e(struct Agent* left, struct Agent* right) {
    (void)right;
    struct Agent* agent0 = mk_agent(AGENT_E);
    struct Agent* agent1 = mk_agent(AGENT_E);
    connect(left, P0, agent0, PMAIN, SRC_REF);
    connect(left, P1, agent1, PMAIN, SRC_REF);
    return;
}

void rule_f_d(struct Agent* left, struct Agent* right) {
    struct Agent* agent0 = mk_agent(AGENT_F);
    struct Agent* agent1 = mk_agent(AGENT_F);
    struct Agent* agent2 = mk_agent(AGENT_D);
    struct Agent* agent3 = mk_agent(AGENT_D);
    connect(agent0, P0, agent2, P0, NO_REF);
    connect(agent0, P1, agent3, P0, NO_REF);
    connect(agent1, P0, agent2, P1, NO_REF);
    connect(agent1, P1, agent3, P1, NO_REF);
    connect(agent0, PMAIN, right, P0, DST_REF);
    connect(agent1, PMAIN, right, P1, DST_REF);
    connect(left, P0, agent2, PMAIN, SRC_REF);
    connect(left, P1, agent3, PMAIN, SRC_REF);
    return;
}

void rule_f_a(struct Agent* left, struct Agent* right) {
    struct Agent* agent0 = mk_agent(AGENT_T);
    connect(agent0, P0, left, P1, DST_REF);
    connect(agent0, P1, right, P0, DST_REF);
    connect(agent0, P2, right, P1, DST_REF);
    connect(left, P0, agent0, PMAIN, SRC_REF);
    return;
}

void rule_f_t(struct Agent* left, struct Agent* right) {
    struct Agent* agent0 = mk_agent(AGENT_Q);
    connect(agent0, P0, left, P0, DST_REF);
    connect(agent0, P1, left, P1, DST_REF);
    connect(agent0, P2, right, P0, DST_REF);
    connect(agent0, P3, right, P2, DST_REF);
    connect(right, P1, agent0, PMAIN, SRC_REF);
    return;
}

void rule_f_q(struct Agent* left, struct Agent* right) {
    struct Agent* agent0 = mk_agent(AGENT_A);
    struct Agent* agent1 = mk_agent(AGENT_A);
    struct Agent* agent2 = mk_agent(AGENT_E);
    struct Agent* agent3 = mk_agent(AGENT_E);
    connect(agent0, P1, agent1, PMAIN, NO_REF);
    connect(agent0, P0, left, P0, DST_REF);
    connect(agent1, P0, left, P1, DST_REF);
    connect(agent1, P1, right, P3, DST_REF);
    connect(right, P2, agent0, PMAIN, SRC_REF);
    connect(right, P0, agent2, PMAIN, SRC_REF);
    connect(right, P1, agent3, PMAIN, SRC_REF);
    return;
}

void print_agent(struct Agent* agent_addr) {
    switch (agent_addr->type) {
        case 0: {
            printf("t");
            break;
        }
        case 1: {
            if (agent_addr->ports[0]->type == 2) {
                printf("t(");
                print_agent(agent_addr->ports[0]);
                printf(")");
            } else {
                printf("t");
                print_agent(agent_addr->ports[0]);
            }
            break;
        }
        case 2: {
            printf("t(");
            print_agent(agent_addr->ports[0]);
            printf(")(");
            print_agent(agent_addr->ports[1]);
            printf(")");
            break;
        }
        case 8: {
            print_agent(agent_addr->ports[0]);
        }
        default: {
            break;
        }
    }
}

void print_tree() {
    // print_agent(&agent_stack.data[0]);
}

char debug_type_to_char(uint8_t type) {
    char types[] = {'L', 'S', 'F', 'E', 'D', 'A', 'T', 'Q', 'I'};
    return types[type];
}

void debug_print_agent_stack() {
    debug("AGENT STACK:\n");
    
    struct AgentStack stack = agent_stack;
    while (stack.current_block->prev != NULL) {
        stack.current_block = stack.current_block->prev;
    }
    struct Agent* ptr = stack.current_block->data;
    while (ptr != stack.next_free_addr) {
        debug("%2lu: %c %d %16lu | %16lu-%d |", (size_t)ptr,
            debug_type_to_char(ptr->type), ptr->port_count,
            (size_t)ptr->pair_stack_addr, (size_t)ptr->ports[PMAIN],
            ptr->port_numbers[PMAIN]);
        for (int i = 0; i < 4; i++) {
            if (i < ptr->port_count) {
                debug(" %16lu-%d", (size_t)ptr->ports[i],
                    ptr->port_numbers[i]);
            // } else {
            //     debug(" (%lu)", (size_t)ptr->ports[i]);
            }
        }
        debug("\n");
        ptr++;

        if (ptr == stack.current_block->end && stack.current_block->next != NULL) {
            stack.current_block = stack.current_block->next;
            ptr = stack.current_block->data;
        }
    }
}

void debug_print_pair_stack() {
    debug("PAIR STACK:\n");

    struct PairStack stack = pair_stack;
    while (stack.current_block->prev != NULL) {
        stack.current_block = stack.current_block->prev;
    }
    struct Pair* ptr = stack.current_block->data;
    while (ptr != stack.next_free_addr) {
        debug("%8lu: %16lu %c %16lu %c\n", (size_t)ptr, (size_t)ptr->agent0,
            debug_type_to_char(ptr->agent0->type), (size_t)ptr->agent1,
            debug_type_to_char(ptr->agent1->type));
        ptr++;

        if (ptr == stack.current_block->end && stack.current_block->next != NULL) {
            stack.current_block = stack.current_block->next;
            ptr = stack.current_block->data;
        }
    }
}

void debug_print(int step_count) {
    debug("\n%d. STEP --------\n", step_count);
    debug_print_agent_stack();
    debug("\n");
    debug_print_pair_stack();
    debug("-------- STEP %d\n\n", step_count);
}

void execute() {
    debug_print(0);
    int i = 1;
    while (step() != 1) {
        debug_print(i++);
    }
}

void init_tree();

int run() {
    // Set up stacks
    init_globals();

    init_tree();

    execute();

    debug("Finished execution: ");
    while (agent_stack.current_block->prev != NULL) {
        agent_stack.current_block = agent_stack.current_block->prev;
    }
    print_agent(&agent_stack.current_block->data[0]);

    return 0;
}


