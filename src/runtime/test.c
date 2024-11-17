#define DEBUG_PRINTS

#include "runtime.c"

void init_tree() {}

void test_rule(char* rule_name) {
    debug("\n>>> Testing rule %s <<<\n", rule_name);
    execute();

    if (agent_stack.next_free_addr != agent_stack.current_block->data ||
        agent_stack.current_block->prev != NULL)
    {
        debug("\nRULE %s IS INCORRECT\n", rule_name);
        exit(EXIT_FAILURE);
    }
    init_globals();
}

void test () {
    init_globals();

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
    test();
}
