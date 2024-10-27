// Plan:
// Write the runtime in C
// Make sure the runtime works properly
// Based on the C version, write the LLVM version

#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

#define AGENT_L    0
#define AGENT_S  257
#define AGENT_F  514
#define AGENT_E  768
#define AGENT_D 1026
#define AGENT_A 1282
#define AGENT_T 1539
#define AGENT_Q 1796
#define AGENT_I 2048

char* agent_type_str = "LSFEDATQI";

#define P0 0
#define P1 1
#define P2 2
#define P3 3
#define PMAIN 4

#define AGENT_STACK_SIZE 256
#define PAIR_STACK_SIZE 32

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

struct AgentStack {
    struct Agent* next_free_addr;
    struct Agent data[AGENT_STACK_SIZE];
};

struct Pair {
    struct Agent* agent0;
    struct Agent* agent1;
};

struct PairStack {
    struct Pair* next_free_addr;
    struct Pair data[PAIR_STACK_SIZE];
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
    agent_stack.next_free_addr = agent_stack.data;

    // pair_stack.data = malloc(PAIR_STACK_SIZE * sizeof(struct Pair));
    pair_stack.next_free_addr = pair_stack.data;
}

// Put an agent on the stack and return the pointer to the agent
struct Agent* store_agent(struct Agent agent) {
    *agent_stack.next_free_addr = agent;
    struct Agent* result = agent_stack.next_free_addr;
    agent_stack.next_free_addr++;
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
}

// Compact the stack, and update pointers
void free_agent(struct Agent* stack_slot) {
    printf("=== Freeing %lu ===\n", (size_t)stack_slot);
    agent_stack.next_free_addr--;
    if (stack_slot == agent_stack.next_free_addr) {
        return;
    }

    printf("    Copying top of stack\n");
    // Copy the top of the stack to the ptr
    *stack_slot = *agent_stack.next_free_addr;

    // Update the parent's and children's ptr
    if (stack_slot->ports[PMAIN] != NULL)  {
        printf("    Updating parent's ptr %lu\n",
            (size_t)stack_slot->ports[PMAIN]);
        stack_slot->ports[PMAIN]->ports[stack_slot->port_numbers[PMAIN]] =
            stack_slot;
    } else {
        printf(">>> [PANIC] AGENT PARENT IS NULL <<<\n");
    }
    for (int i = 0; i < stack_slot->port_count; i++) {
        if (stack_slot->ports[i] != NULL)  {
            printf("    Updating child's ptr %lu\n",
                (size_t)stack_slot->ports[i]);
            stack_slot->ports[i]->ports[stack_slot->port_numbers[i]] =
                stack_slot;
        } else {
            printf(">>> [PANIC] AGENT CHILD %d / %d IS NULL <<<\n", i,
                stack_slot->port_count);
        }
    }

    printf("    Updating stack ptr %lu\n", (size_t)stack_slot->pair_stack_addr);
    // Update the stack's ptr
    if (stack_slot->pair_stack_addr != NULL) {
        *(stack_slot->pair_stack_addr) = stack_slot;
    }
}

// Pop an equation off the stack, execute the corresponding rule, and free the
// agents afterwards
uint8_t step() {
    printf("Pop pair stack\n");
    if (pair_stack.next_free_addr == pair_stack.data) {
        return 1;
    }

    pair_stack.next_free_addr--;
    struct Pair pair = *pair_stack.next_free_addr;

    struct Agent* agent0_addr = pair.agent0;
    struct Agent* agent1_addr = pair.agent1;
    if (agent0_addr->type == 8 || agent1_addr->type == 8) {
        return 1;
    }

    uint8_t rule_index = agent0_addr->type * 5 + agent1_addr->type - 3;
    printf("Execute rule %d\n", rule_index);
    (*rule_table[rule_index])(agent0_addr, agent1_addr);
    printf("Free agents\n");
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
    connect(agent0, P3, right, P1, DST_REF);
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
            printf("t(");
            print_agent(agent_addr->ports[0]);
            printf(")");
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
    print_agent(&agent_stack.data[0]);
}

char debug_type_to_char(uint8_t type) {
    char types[] = {'L', 'S', 'F', 'E', 'D', 'A', 'T', 'Q', 'I'};
    return types[type];
}

void debug_print_agent_stack() {
    printf("AGENT STACK:\n");
    
    struct Agent* ptr = &agent_stack.data[0];
    while (ptr != agent_stack.next_free_addr) {
        printf("%2lu: %c %d %16lu | %16lu-%d |", (size_t)ptr,
            debug_type_to_char(ptr->type), ptr->port_count,
            (size_t)ptr->pair_stack_addr, (size_t)ptr->ports[PMAIN],
            ptr->port_numbers[PMAIN]);
        for (int i = 0; i < 4; i++) {
            if (i < ptr->port_count) {
                printf(" %16lu-%d", (size_t)ptr->ports[i],
                    ptr->port_numbers[i]);
            // } else {
            //     printf(" (%lu)", (size_t)ptr->ports[i]);
            }
        }
        printf("\n");
        ptr++;
    }
}

void debug_print_pair_stack() {
    printf("PAIR STACK:\n");

    struct Pair* ptr = &pair_stack.data[0];
    while (ptr != pair_stack.next_free_addr) {
        printf("%8lu: %16lu %c %16lu %c\n", (size_t)ptr, (size_t)ptr->agent0,
            debug_type_to_char(ptr->agent0->type), (size_t)ptr->agent1,
            debug_type_to_char(ptr->agent1->type));
        ptr++;
    }
}

void debug_print(int step_count) {
    printf("\n%d. STEP --------\n", step_count);
    debug_print_agent_stack();
    printf("\n");
    debug_print_pair_stack();
    printf("-------- STEP\n\n");
}

void execute() {
    debug_print(0);
    int i = 1;
    while (step() != 1) {
        debug_print(i++);
    }
}

void test_rule(char* rule_name) {
    printf("\n>>> Testing rule %s <<<\n", rule_name);
    execute();

    if (agent_stack.next_free_addr != agent_stack.data) {
        printf("\nRULE %s IS INCORRECT\n", rule_name);
        exit(EXIT_FAILURE);
    }
    init_globals();
}

void test () {
    struct Agent* agent0;
    struct Agent* agent1;
    struct Agent* agent2;
    struct Agent* agent3;
    struct Agent* agent4;
    struct Agent* agent5;
    struct Agent* agent6;
    struct Agent* agent7;

    agent0 = mk_agent(AGENT_L);
    agent1 = mk_agent(AGENT_E);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    test_rule("L-E");
    
    agent0 = mk_agent(AGENT_S);
    agent1 = mk_agent(AGENT_E);
    agent2 = mk_agent(AGENT_L);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent2, PMAIN, agent0, P0, NO_REF);
    test_rule("S-E");

    agent0 = mk_agent(AGENT_F);
    agent1 = mk_agent(AGENT_E);
    agent2 = mk_agent(AGENT_L);
    agent3 = mk_agent(AGENT_L);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent2, PMAIN, agent0, P0, NO_REF);
    connect(agent3, PMAIN, agent0, P1, NO_REF);
    test_rule("F-E");

    agent0 = mk_agent(AGENT_L);
    agent1 = mk_agent(AGENT_D);
    agent2 = mk_agent(AGENT_E);
    agent3 = mk_agent(AGENT_E);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent1, P0, agent2, PMAIN, NO_REF);
    connect(agent1, P1, agent3, PMAIN, NO_REF);
    test_rule("L-D");

    agent0 = mk_agent(AGENT_S);
    agent1 = mk_agent(AGENT_D);
    agent2 = mk_agent(AGENT_E);
    agent3 = mk_agent(AGENT_E);
    agent4 = mk_agent(AGENT_L);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent1, P0, agent2, PMAIN, NO_REF);
    connect(agent1, P1, agent3, PMAIN, NO_REF);
    connect(agent4, PMAIN, agent0, P0, NO_REF);
    test_rule("S-D");

    agent0 = mk_agent(AGENT_F);
    agent1 = mk_agent(AGENT_D);
    agent2 = mk_agent(AGENT_E);
    agent3 = mk_agent(AGENT_E);
    agent4 = mk_agent(AGENT_L);
    agent5 = mk_agent(AGENT_L);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent1, P0, agent2, PMAIN, NO_REF);
    connect(agent1, P1, agent3, PMAIN, NO_REF);
    connect(agent4, PMAIN, agent0, P0, NO_REF);
    connect(agent5, PMAIN, agent0, P1, NO_REF);
    test_rule("F-D");

    agent0 = mk_agent(AGENT_L);
    agent1 = mk_agent(AGENT_A);
    agent2 = mk_agent(AGENT_L);
    agent3 = mk_agent(AGENT_E);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent2, PMAIN, agent1, P0, NO_REF);
    connect(agent3, PMAIN, agent1, P1, NO_REF);
    test_rule("L-A");

    agent0 = mk_agent(AGENT_S);
    agent1 = mk_agent(AGENT_A);
    agent2 = mk_agent(AGENT_L);
    agent3 = mk_agent(AGENT_E);
    agent4 = mk_agent(AGENT_L);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent2, PMAIN, agent1, P0, NO_REF);
    connect(agent3, PMAIN, agent1, P1, NO_REF);
    connect(agent4, PMAIN, agent0, P0, NO_REF);
    test_rule("S-A");

    agent0 = mk_agent(AGENT_F);
    agent1 = mk_agent(AGENT_A);
    agent2 = mk_agent(AGENT_L);
    agent3 = mk_agent(AGENT_E);
    agent4 = mk_agent(AGENT_L);
    agent5 = mk_agent(AGENT_L);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent2, PMAIN, agent1, P0, NO_REF);
    connect(agent3, PMAIN, agent1, P1, NO_REF);
    connect(agent4, PMAIN, agent0, P0, NO_REF);
    connect(agent5, PMAIN, agent0, P1, NO_REF);
    test_rule("F-A");

    agent0 = mk_agent(AGENT_L);
    agent1 = mk_agent(AGENT_T);
    agent2 = mk_agent(AGENT_L);
    agent3 = mk_agent(AGENT_L);
    agent4 = mk_agent(AGENT_E);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent2, PMAIN, agent1, P0, NO_REF);
    connect(agent3, PMAIN, agent1, P1, NO_REF);
    connect(agent4, PMAIN, agent1, P2, NO_REF);
    test_rule("L-T");

    agent0 = mk_agent(AGENT_S);
    agent1 = mk_agent(AGENT_T);
    agent2 = mk_agent(AGENT_L);
    agent3 = mk_agent(AGENT_L);
    agent4 = mk_agent(AGENT_E);
    agent5 = mk_agent(AGENT_L);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent2, PMAIN, agent1, P0, NO_REF);
    connect(agent3, PMAIN, agent1, P1, NO_REF);
    connect(agent4, PMAIN, agent1, P2, NO_REF);
    connect(agent5, PMAIN, agent0, P0, NO_REF);
    test_rule("S-T");

    agent0 = mk_agent(AGENT_F);
    agent1 = mk_agent(AGENT_T);
    agent2 = mk_agent(AGENT_L);
    agent3 = mk_agent(AGENT_L);
    agent4 = mk_agent(AGENT_E);
    agent5 = mk_agent(AGENT_L);
    agent6 = mk_agent(AGENT_L);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent2, PMAIN, agent1, P0, NO_REF);
    connect(agent3, PMAIN, agent1, P1, NO_REF);
    connect(agent4, PMAIN, agent1, P2, NO_REF);
    connect(agent5, PMAIN, agent0, P0, NO_REF);
    connect(agent6, PMAIN, agent0, P1, NO_REF);
    test_rule("F-T");

    agent0 = mk_agent(AGENT_L);
    agent1 = mk_agent(AGENT_Q);
    agent2 = mk_agent(AGENT_L);
    agent3 = mk_agent(AGENT_L);
    agent4 = mk_agent(AGENT_L);
    agent5 = mk_agent(AGENT_E);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent2, PMAIN, agent1, P0, NO_REF);
    connect(agent3, PMAIN, agent1, P1, NO_REF);
    connect(agent4, PMAIN, agent1, P2, NO_REF);
    connect(agent5, PMAIN, agent1, P3, NO_REF);
    test_rule("L-Q");

    agent0 = mk_agent(AGENT_S);
    agent1 = mk_agent(AGENT_Q);
    agent2 = mk_agent(AGENT_L);
    agent3 = mk_agent(AGENT_L);
    agent4 = mk_agent(AGENT_L);
    agent5 = mk_agent(AGENT_E);
    agent6 = mk_agent(AGENT_L);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent2, PMAIN, agent1, P0, NO_REF);
    connect(agent3, PMAIN, agent1, P1, NO_REF);
    connect(agent4, PMAIN, agent1, P2, NO_REF);
    connect(agent5, PMAIN, agent1, P3, NO_REF);
    connect(agent6, PMAIN, agent0, P0, NO_REF);
    test_rule("S-Q");

    agent0 = mk_agent(AGENT_F);
    agent1 = mk_agent(AGENT_Q);
    agent2 = mk_agent(AGENT_L);
    agent3 = mk_agent(AGENT_L);
    agent4 = mk_agent(AGENT_L);
    agent5 = mk_agent(AGENT_E);
    agent6 = mk_agent(AGENT_L);
    agent7 = mk_agent(AGENT_L);
    connect(agent0, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent2, PMAIN, agent1, P0, NO_REF);
    connect(agent3, PMAIN, agent1, P1, NO_REF);
    connect(agent4, PMAIN, agent1, P2, NO_REF);
    connect(agent5, PMAIN, agent1, P3, NO_REF);
    connect(agent6, PMAIN, agent0, P0, NO_REF);
    connect(agent7, PMAIN, agent0, P1, NO_REF);
    test_rule("F-Q");
}

int main() {
    // Set up stacks
    init_globals();

    // test();

    // Set up initial tree for "tttt" (result should be "t")
    // struct Agent* agent0 = mk_agent(0, AGENT_I);
    // struct Agent* agent1 = mk_agent(0, AGENT_A);
    // struct Agent* agent2 = mk_agent(0, AGENT_L);
    // struct Agent* agent3 = mk_agent(0, AGENT_L);
    // struct Agent* agent4 = mk_agent(0, AGENT_A);
    // load(1, agent2);
    // load(2, agent3);
    // push(1, 0);
    // connect(0, 0, 2, 4);
    // struct Agent* agent5 = mk_agent(0, AGENT_L);
    // struct Agent* agent6 = mk_agent(0, AGENT_A);
    // load(1, agent4);
    // load(2, agent5);
    // connect(1, 1, 0, 4);
    // connect(0, 0, 2, 4);
    // struct Agent* agent7 = mk_agent(0, AGENT_L);
    // load(0, agent1);
    // load(1, agent6);
    // load(2, agent7);
    // connect(1, 1, 0, 4);
    // connect(0, 0, 2, 4);
    // load(0, agent0);
    // load(1, agent1);
    // connect(1, 1, 0, 0);

    // Set up initial tree for "t(tt)(tt)t" (result should be "tt(ttt)")
    // struct Agent* agent0 = mk_agent(0, AGENT_I);
    // struct Agent* agent1 = mk_agent(0, AGENT_A);
    // struct Agent* agent2 = mk_agent(0, AGENT_A);
    // struct Agent* agent3 = mk_agent(0, AGENT_A);
    // struct Agent* agent4 = mk_agent(0, AGENT_A);
    // struct Agent* agent5 = mk_agent(0, AGENT_L);
    // load(0, agent4);
    // struct Agent* agent6 = mk_agent(1, AGENT_L);
    // load(2, agent5);
    // push(1, 0);
    // connect(0, 0, 2, 4);
    // load(0, agent3);
    // struct Agent* agent7 = mk_agent(1, AGENT_L);
    // load(2, agent4);
    // push(1, 0);
    // connect(0, 0, 2, 1);
    // struct Agent* agent8 = mk_agent(0, AGENT_A);
    // struct Agent* agent9 = mk_agent(0, AGENT_L);
    // load(0, agent8);
    // struct Agent* agent10 = mk_agent(1, AGENT_L);
    // load(2, agent9);
    // push(1, 0);
    // connect(0, 0, 2, 4);
    // load(0, agent2);
    // load(1, agent3);
    // load(2, agent8);
    // connect(0, 4, 1, 1);
    // connect(0, 0, 2, 1);
    // struct Agent* agent11 = mk_agent(0, AGENT_L);
    // load(0, agent1);
    // load(1, agent2);
    // load(2, agent11);
    // connect(0, 4, 1, 1);
    // connect(0, 0, 2, 4);
    // load(0, agent0);
    // load(1, agent1);
    // connect(1, 1, 0, 0);

    struct Agent* agent0 = mk_agent(AGENT_I);
    struct Agent* agent1 = mk_agent(AGENT_A);
    struct Agent* agent2 = mk_agent(AGENT_F);
    struct Agent* agent3 = mk_agent(AGENT_S);
    struct Agent* agent4 = mk_agent(AGENT_L);
    connect(agent3, P0, agent4, PMAIN, NO_REF);
    struct Agent* agent5 = mk_agent(AGENT_S);
    struct Agent* agent6 = mk_agent(AGENT_L);
    connect(agent5, P0, agent6, PMAIN, NO_REF);
    connect(agent2, P0, agent3, PMAIN, NO_REF);
    connect(agent2, P1, agent5, PMAIN, NO_REF);
    struct Agent* agent7 = mk_agent(AGENT_L);
    connect(agent2, PMAIN, agent1, PMAIN, NO_REF);
    connect(agent1, P0, agent7, PMAIN, NO_REF);
    connect(agent1, P1, agent0, P0, NO_REF);

    execute();

    printf("Finished execution: ");
    print_agent(&agent_stack.data[0]);

    return 0;
}
