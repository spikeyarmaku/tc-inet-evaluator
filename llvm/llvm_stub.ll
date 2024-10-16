; Assumptions: 64-bit arch
; %Word = type i64

; Agent types - type (i8) + port num (i8)
@agent_name = constant i16 0    ; 256 * 0 + 0
@agent_l = constant i16  256    ; 256 * 1 + 0
@agent_s = constant i16  513    ; 256 * 2 + 1
@agent_f = constant i16  770    ; 256 * 3 + 2
@agent_e = constant i16 1024    ; 256 * 4 + 0
@agent_d = constant i16 1282    ; 256 * 5 + 2
@agent_a = constant i16 1538    ; 256 * 6 + 2
@agent_t = constant i16 1795    ; 256 * 7 + 3
@agent_q = constant i16 2052    ; 256 * 8 + 4

; The Agent type contains:
; - type (i8)
; - port count (i8)
; - ptr to its parent's port
; - ptr to its slot on the pair stack
; - ptr to connected agents
; It stores a ptr to its parent and to a part in the active stack so that when
; an agent is freed and this agent is moved to its place, it can update the ptr
; pointing to itself in both places
%Agent = type {i8, i8, %Agent**, %Agent**, [4 x %Agent*]}
; Agent stack - ptr to the next empty space, ptr to the start of the stack
%AgentStack = type {%AgentStack*, [1000 x %Agent]}
@agent_stack = global %AgentStack poison

; Active stack
%Pair = type {%Agent*, %Agent*}
; Pair stack - ptr to the next empty space, ptr to the start of the stack
%PairStack = type {%Pair*, [20 x %Pair]}
@pair_stack = global %PairStack poison

; TreeCalc Inet registers (16)
@reg = global [16 x %Agent*] poison

define void @rule_l_e() {
    ret void
}

define void @rule_l_d() {
    call ptr @mk_agent(i8 8, ptr @agent_l)
    call ptr @mk_agent(i8 9, ptr @agent_l)
    call void @push(i8 8, i8 4)
    call void @push(i8 9, i8 5)
    ret void
}

define void @rule_l_a() {
    call ptr @mk_agent(i8 8, ptr @agent_s)
    call void @connect(i8 8, i8 0, i8 4)
    call void @push(i8 8, i8 5)
    ret void
}

define void @rule_l_t() {
    call ptr @mk_agent(i8 8, ptr @agent_e)
    call void @push(i8 4, i8 6)
    call void @push(i8 5, i8 8)
    ret void
}

define void @rule_l_q() {
    call ptr @mk_agent(i8 8, ptr @agent_e)
    call ptr @mk_agent(i8 9, ptr @agent_e)
    call void @push(i8 4, i8 7)
    call void @push(i8 5, i8 8)
    call void @push(i8 6, i8 9)
    ret void
}

define void @rule_s_e() {
    call ptr @mk_agent(i8 8, ptr @agent_e)
    call void @push(i8 0, i8 8)
    ret void
}

define void @rule_s_d() {
    call ptr @mk_agent(i8 8, ptr @agent_d)
    call ptr @mk_agent(i8 9, ptr @agent_name)
    call ptr @mk_agent(i8 10, ptr @agent_name)
    call void @connect(i8 8, i8 0, i8 9)
    call void @connect(i8 8, i8 1, i8 10)
    call void @push(i8 0, i8 8)
    call ptr @mk_agent(i8 11, ptr @agent_s)
    call void @connect(i8 11, i8 0, i8 9)
    call void @push(i8 11, i8 4)
    call ptr @mk_agent(i8 12, ptr @agent_s)
    call void @connect(i8 12, i8 0, i8 10)
    call void @push(i8 12, i8 5)
    ret void
}

define void @rule_s_a() {
    call ptr @mk_agent(i8 8, ptr @agent_f)
    call void @connect(i8 8, i8 0, i8 0)
    call void @connect(i8 8, i8 1, i8 4)
    call void @push(i8 8, i8 5)
    ret void
}

define void @rule_s_t() {
    call ptr @mk_agent(i8 8, ptr @agent_a)
    call ptr @mk_agent(i8 9, ptr @agent_a)
    call ptr @mk_agent(i8 10, ptr @agent_name)
    call ptr @mk_agent(i8 11, ptr @agent_name)
    call void @push(i8 0, i8 8)
    call void @connect(i8 8, i8 0, i8 10)
    call void @connect(i8 8, i8 1, i8 9)
    call void @connect(i8 9, i8 0, i8 11)
    call void @connect(i8 9, i8 1, i8 6)
    call ptr @mk_agent(i8 12, ptr @agent_d)
    call ptr @mk_agent(i8 13, ptr @agent_name)
    call void @push(i8 5, i8 12)
    call void @connect(i8 12, i8 0, i8 13)
    call void @connect(i8 12, i8 1, i8 10)
    call ptr @mk_agent(i8 14, ptr @agent_a)
    call void @push(i8 4, i8 14)
    call void @connect(i8 14, i8 0, i8 13)
    call void @connect(i8 14, i8 1, i8 11)
    ret void
}

define void @rule_s_q() {
    call ptr @mk_agent(i8 8, ptr @agent_a)
    call void @push(i8 5, i8 8)
    call void @connect(i8 8, i8 0, i8 0)
    call void @connect(i8 8, i8 1, i8 7)
    call ptr @mk_agent(i8 9, ptr @agent_e)
    call void @push(i8 4, i8 9)
    call ptr @mk_agent(i8 10, ptr @agent_e)
    call void @push(i8 6, i8 10)
    ret void
}

define void @rule_f_l() {
    call ptr @mk_agent(i8 8, ptr @agent_e)
    call void @push(i8 0, i8 8)
    call ptr @mk_agent(i8 9, ptr @agent_e)
    call void @push(i8 1, i8 9)
    ret void
}

define void @rule_f_d() {
    call ptr @mk_agent(i8 8, ptr @agent_f)
    call ptr @mk_agent(i8 9, ptr @agent_name)
    call ptr @mk_agent(i8 10, ptr @agent_name)
    call void @connect(i8 8, i8 0, i8 9)
    call void @connect(i8 8, i8 1, i8 10)
    call void @push(i8 8, i8 5)
    call ptr @mk_agent(i8 11, ptr @agent_f)
    call ptr @mk_agent(i8 12, ptr @agent_name)
    call ptr @mk_agent(i8 13, ptr @agent_name)
    call void @connect(i8 11, i8 0, i8 12)
    call void @connect(i8 11, i8 1, i8 13)
    call void @push(i8 11, i8 4)
    call ptr @mk_agent(i8 14, ptr @agent_d)
    call void @connect(i8 14, i8 0, i8 12)
    call void @connect(i8 14, i8 1, i8 9)
    call void @push(i8 0, i8 14)
    call ptr @mk_agent(i8 15, ptr @agent_d)
    call void @connect(i8 15, i8 0, i8 13)
    call void @connect(i8 15, i8 1, i8 10)
    call void @push(i8 1, i8 15)
    ret void
}

define void @rule_f_a() {
    call ptr @mk_agent(i8 8, ptr @agent_t)
    call void @connect(i8 8, i8 0, i8 1)
    call void @connect(i8 8, i8 1, i8 4)
    call void @connect(i8 8, i8 2, i8 5)
    call void @push(i8 0, i8 8)
    ret void
}

define void @rule_f_t() {
    call ptr @mk_agent(i8 8, ptr @agent_q)
    call void @push(i8 5, i8 8)
    call void @connect(i8 8, i8 0, i8 0)
    call void @connect(i8 8, i8 1, i8 1)
    call void @connect(i8 8, i8 2, i8 4)
    call void @connect(i8 8, i8 3, i8 6)
    ret void
}

define void @rule_f_q() {
    call ptr @mk_agent(i8 8, ptr @agent_a)
    call ptr @mk_agent(i8 9, ptr @agent_a)
    call void @push(i8 6, i8 8)
    call void @connect(i8 8, i8 0, i8 0)
    call void @connect(i8 8, i8 1, i8 9)
    call void @connect(i8 9, i8 0, i8 1)
    call void @connect(i8 9, i8 1, i8 7)
    call ptr @mk_agent(i8 10, ptr @agent_e)
    call void @push(i8 4, i8 10)
    call ptr @mk_agent(i8 11, ptr @agent_e)
    call void @push(i8 5, i8 11)
    ret void
}

@rule_table = global [15 x void ()*]
    [
        void ()* @rule_l_e, void ()* @rule_l_d, void ()* @rule_l_a, void ()* @rule_l_t, void ()* @rule_l_q,
        void ()* @rule_s_e, void ()* @rule_s_d, void ()* @rule_s_a, void ()* @rule_s_t, void ()* @rule_s_q,
        void ()* @rule_f_l, void ()* @rule_f_d, void ()* @rule_f_a, void ()* @rule_f_t, void ()* @rule_f_q
    ]

; Store an agent on the stack
define ptr @store_agent(%Agent %agent) {
    ; Get pointer to the new empty space on stack
    %stack_ptr_addr = getelementptr %AgentStack, ptr @agent_stack, i32 0, i32 0
    %stack_ptr = load ptr, ptr %stack_ptr_addr

    ; Store the agent
    store %Agent %agent, ptr %stack_ptr

    ; Bump the stack pointer
    %new_stack_ptr = getelementptr %Agent, ptr %stack_ptr, i32 1
    store ptr %new_stack_ptr, ptr %stack_ptr_addr

    ret ptr %stack_ptr
}

; Store an agent on the agent stack, copy its address to the register, and move
; the stack pointer. Return the pointer to the agent
define ptr @mk_agent(i8 %reg_index, ptr %agent_type_const) {
    ; Construct the agent's id
    %value = load i16, ptr %agent_type_const
    %port_count = trunc i16 %value to i8
    %agent_type.i8 = lshr i16 %value, 8
    %agent_type = trunc i16 %agent_type.i8 to i8

    ; Construct the struct elements
    %agent.0 = insertvalue %Agent poison, i8 %agent_type, 0
    %agent.1 = insertvalue %Agent %agent.0, i8 %port_count, 1
    %agent.2 = insertvalue %Agent %agent.1, ptr zeroinitializer, 2
    %agent.3 = insertvalue %Agent %agent.2, ptr zeroinitializer, 3
    %agent.4 = insertvalue %Agent %agent.3, [4 x ptr] zeroinitializer, 4

    ; Store the values
    %stack_ptr = call ptr @store_agent(%Agent %agent.4)

    ; Copy the address to the register
    %reg_ptr = getelementptr ptr, ptr @reg, i8 %reg_index
    store ptr %stack_ptr, ptr %reg_ptr

    ret ptr %stack_ptr
}

; Put address onto register
define void @load(i8 %reg_index, ptr %agent_addr) {
    %reg_ptr = getelementptr %Agent*, ptr @reg, i8 %reg_index
    store ptr %agent_addr, ptr %reg_ptr

    ret void
}

; Connect a port of the agent on the register to the principal port of the agent
; on the agent stack
define void @connect(i8 %reg_index_0, i8 %portnum, i8 %reg_index_1) {
    %reg_ptr_0 = getelementptr %Agent*, ptr @reg, i8 %reg_index_0
    %reg_ptr_1 = getelementptr %Agent*, ptr @reg, i8 %reg_index_1
    %agent0_ptr = load ptr, ptr %reg_ptr_0
    %agent1_ptr = load ptr, ptr %reg_ptr_1
    
    ; Change source agent's port value
    %agent0_port_ptr = getelementptr %Agent, ptr %agent0_ptr, i64 0, i32 4, i8 %portnum
    store ptr %agent1_ptr, ptr %agent0_port_ptr

    ; Change the target agent's parent value to the source agent's port address
    %agent1_parent_ptr = getelementptr %Agent, ptr %agent1_ptr, i64 0, i32 2
    store ptr %agent0_port_ptr, ptr %agent1_parent_ptr

    ret void
}

define ptr @store_pair(%Pair %pair) {
    ; Get pointer to the new empty space on stack
    %stack_ptr_addr = getelementptr %PairStack, ptr @pair_stack, i32 0, i32 0
    %stack_ptr = load ptr, ptr %stack_ptr_addr

    ; Store the pair
    store %Pair %pair, ptr %stack_ptr

    ; Bump the stack pointer
    %new_stack_ptr = getelementptr %Pair, ptr %stack_ptr, i32 1
    store ptr %new_stack_ptr, ptr %stack_ptr_addr

    ret ptr %stack_ptr
}

; Add a new pair to the pair stack
define void @push(i8 %reg_index0, i8 %reg_index1) {
    ; write the new element
    %reg_ptr_0 = getelementptr %Agent*, ptr @reg, i8 %reg_index0
    %reg_ptr_1 = getelementptr %Agent*, ptr @reg, i8 %reg_index1
    %agent0_ptr = load ptr, ptr %reg_ptr_0
    %agent1_ptr = load ptr, ptr %reg_ptr_1
    %new_pair.0 = insertvalue %Pair poison, ptr %agent0_ptr, 0
    %new_pair.1 = insertvalue %Pair %new_pair.0, ptr %agent1_ptr, 1
    
    ; Store the new element
    %pair_ptr = call ptr @store_pair(%Pair %new_pair.1)

    ; Update agents' pair ptrs
    %pair_ptr.0 = getelementptr %Pair, ptr %pair_ptr, i32 0
    %agent0_pair_ptr = getelementptr %Agent, ptr %agent0_ptr, i64 0, i32 3
    store ptr %pair_ptr.0, ptr %agent0_pair_ptr
    %pair_ptr.1 = getelementptr %Pair, ptr %pair_ptr, i32 1
    %agent1_pair_ptr = getelementptr %Agent, ptr %agent1_ptr, i64 0, i32 3
    store ptr %pair_ptr.1, ptr %agent1_pair_ptr

    ret void
}

; Copy the top of the agent stack to the provided address, and update pointers
define void @free_agent(ptr %agent_ptr) {
    ; Decrease the stack pointer
    %stack_ptr_addr = getelementptr %AgentStack, ptr @agent_stack, i32 0, i32 0
    %stack_ptr_old = load ptr, ptr %stack_ptr_addr
    %stack_ptr = getelementptr %Agent, ptr %stack_ptr_old, i32 -1
    store ptr %stack_ptr, ptr %stack_ptr_addr

    ; If the agent is deleted from the top of the stack, do nothing
    %cond_top = icmp eq ptr %agent_ptr, %stack_ptr
    br i1 %cond_top, label %end, label %swap

swap:
    ; Get the agent from the top of the stack
    %agent = load %Agent, ptr %stack_ptr

    ; Store it at the provided address
    store %Agent %agent, ptr %agent_ptr

    ; Update its parent
    %parent_agent_ptr = extractvalue %Agent %agent, 2
    ; Check against null ptr (agent has no parent)
    %cond_parent = icmp eq ptr %parent_agent_ptr, null
    br i1 %cond_parent, label %jump, label %write
write:
    store ptr %agent_ptr, ptr %parent_agent_ptr
    ; Update its ptr on the pair stack
    %pair_stack_ptr = extractvalue %Agent %agent, 3
    ; Check against null ptr (agent is not on pair stack)
    %cond_pair_stack = icmp eq ptr %pair_stack_ptr, null
    br i1 %cond_pair_stack, label %jump, label %write_pair

write_pair:
    store ptr %agent_ptr, ptr %pair_stack_ptr
    br label %jump

jump:
    ; Update its children
    %aux_port_count = extractvalue %Agent %agent, 1
    %counter_ptr = alloca i8
    store i8 0, ptr %counter_ptr
    br label %check
check:
    ; Load counter and check if it is equal to the number of ports
    %counter = load i8, ptr %counter_ptr
    %cond = icmp eq i8 %aux_port_count, %counter
    br i1 %cond, label %end, label %overwrite
overwrite:
    %port_num_ptr = getelementptr %Agent, ptr %agent_ptr, i64 0, i32 4, i8 %counter
    %child_ptr = load ptr, ptr %port_num_ptr
    %child_parent_ptr = getelementptr %Agent, ptr %child_ptr, i64 0, i32 2
    store ptr %port_num_ptr, ptr %child_parent_ptr

    %p = ptrtoint ptr %port_num_ptr to i64
    ; call void @print_i64(i64 %p)

    ; Bump the counter
    %counter.1 = add i8 %counter, 1
    store i8 %counter.1, ptr %counter_ptr
    br label %check
end:
    ret void
}

; Set up the stack pointers for the agent stack and the pair stack
define void @init() {
    %agent_stack_ptr = getelementptr %AgentStack, ptr @agent_stack, i64 0, i32 0
    %agent_stack_addr = getelementptr %AgentStack, ptr @agent_stack, i64 0, i32 1, i64 0
    store ptr %agent_stack_addr, ptr %agent_stack_ptr

    %pair_stack_ptr = getelementptr %PairStack, ptr @pair_stack, i64 0, i32 0
    %pair_stack_addr = getelementptr %PairStack, ptr @pair_stack, i64 0, i32 1, i64 0
    store ptr %pair_stack_addr, ptr %pair_stack_ptr

    ret void
}

; Set up registers based on the ports and register index provided
define void @set_up_regs(i8 %agent_port_count, ptr %agent_ports_ptr, i8 %reg_index) {
    %port_counter_ptr = alloca i8
    store i8 0, ptr %port_counter_ptr
    
    %reg_counter_ptr = alloca i8
    store i8 %reg_index, ptr %reg_counter_ptr
    
    br label %reg_check

reg_check:
    ; Check if counter == port_count
    %port_counter.0 = load i8, ptr %port_counter_ptr
    %port_counter_cond = icmp eq i8 %port_counter.0, %agent_port_count
    br i1 %port_counter_cond, label %end, label %reg_fill

reg_fill:
    ; Load reg counter, set up reg_ptr
    %reg_counter = load i8, ptr %reg_counter_ptr
    %reg_ptr = getelementptr %Agent*, ptr @reg, i8 %reg_counter
    ; Load port value
    %port_ptr = getelementptr %Agent*, ptr %agent_ports_ptr, i8 %port_counter.0
    %port_value = load ptr, ptr %port_ptr
    ; Read and store current port value at reg_ptr
    store ptr %port_value, ptr %reg_ptr
    ; Increment port counter
    %port_counter.1 = add i8 %port_counter.0, 1
    store i8 %port_counter.1, ptr %port_counter_ptr
    ; Increment reg counter
    %reg_counter.1 = add i8 %reg_counter, 1
    store i8 %reg_counter.1, ptr %reg_counter_ptr
    br label %reg_check

end:
    ret void
}

define void @execute() {
    br label %start

start:
    ; Check if there are pairs on the pair stack
    %ps_curr_ptr_ptr = getelementptr %PairStack, ptr @pair_stack, i64 0, i32 0
    %ps_start_ptr = getelementptr %PairStack, ptr @pair_stack, i64 0, i32 1, i64 0
    %ps_curr_ptr = load ptr, ptr %ps_curr_ptr_ptr

    ; %ps_start = ptrtoint ptr %ps_start_ptr to i64
    ; call void @print_i64(i64 %ps_start)
    ; %ps_curr = ptrtoint ptr %ps_curr_ptr to i64
    ; call void @print_i64(i64 %ps_curr)

    %stack_empty = icmp eq ptr %ps_curr_ptr, %ps_start_ptr
    br i1 %stack_empty, label %end, label %exec

exec:
    ; If there is a pair on the stack, decrement the stack pointer, and get the
    ; top element
    %ps_curr_ptr.1 = getelementptr %Pair, ptr %ps_curr_ptr, i64 -1
    store ptr %ps_curr_ptr.1, ptr %ps_curr_ptr_ptr
    %p = load %Pair, ptr %ps_curr_ptr.1

    ; Get the agent types
    %agent0_ptr = extractvalue %Pair %p, 0
    %agent1_ptr = extractvalue %Pair %p, 1
    %agent0 = load %Agent, ptr %agent0_ptr
    %agent1 = load %Agent, ptr %agent1_ptr
    %agent0_type = extractvalue %Agent %agent0, 0
    %agent1_type = extractvalue %Agent %agent1, 0
    %agent0_port_count = extractvalue %Agent %agent0, 1
    %agent1_port_count = extractvalue %Agent %agent1, 1
    %agent0_ports_ptr = getelementptr %Agent, ptr %agent0_ptr, i64 0, i32 4
    %agent1_ports_ptr = getelementptr %Agent, ptr %agent1_ptr, i64 0, i32 4

    ; Check if any of the agents are names or indirections (name with 0 or 1
    ; port count)
    %agent0_is_name = icmp eq i8 %agent0_type, 0
    br i1 %agent0_is_name, label %agent0_name, label %agent1_check

agent0_name:
    %agent0_is_indir = icmp eq i8 %agent0_port_count, 1
    %agent0_port0_ptr = getelementptr %Agent*, ptr %agent0_ports_ptr, i64 0
    ; Change port count of agent
    %agent0_port_count_ptr = getelementptr %Agent, ptr %agent0_ptr, i64 0, i32 1
    store i8 1, ptr %agent0_port_count_ptr
    br i1 %agent0_is_indir, label %agent0_indir, label %agent0_name_cont

agent0_indir:
    %agent0_port0 = load ptr, ptr %agent0_port0_ptr
    %new_pair0.0 = insertvalue %Pair poison, ptr %agent0_port0, 0
    %new_pair0.1 = insertvalue %Pair %new_pair0.0, ptr %agent1_ptr, 1
    call void @store_pair(%Pair %new_pair0.1)
    ; Theoretically the pair stack ptrs stay the same, so no need to update
    call void @free_agent(ptr %agent0_ptr)
    br label %end

agent0_name_cont:
    store ptr %agent1_ptr, ptr %agent0_port0_ptr
    br label %end

agent1_check:
    %agent1_is_name = icmp eq i8 %agent1_type, 0
    br i1 %agent1_is_name, label %agent1_name, label %rule

agent1_name:
    %agent1_is_indir = icmp eq i8 %agent1_port_count, 1
    %agent1_port0_ptr = getelementptr %Agent*, ptr %agent1_ports_ptr, i64 0
    ; Change port count of agent
    %agent1_port_count_ptr = getelementptr %Agent, ptr %agent1_ptr, i64 0, i32 1
    store i8 1, ptr %agent1_port_count_ptr
    br i1 %agent1_is_indir, label %agent1_indir, label %agent1_name_cont

agent1_indir:
    %agent1_port0 = load ptr, ptr %agent1_port0_ptr
    %new_pair1.0 = insertvalue %Pair poison, ptr %agent0_ptr, 0
    %new_pair1.1 = insertvalue %Pair %new_pair1.0, ptr %agent1_port0, 1
    call void @store_pair(%Pair %new_pair1.1)
    ; Theoretically the pair stack ptrs stay the same, so no need to update
    call void @free_agent(ptr %agent1_ptr)
    br label %end

agent1_name_cont:
    store ptr %agent0_ptr, ptr %agent1_port0_ptr
    br label %end

rule:
    ; Set up regs
    call void @set_up_regs(i8 %agent0_port_count, ptr %agent0_ports_ptr, i8 0)
    call void @set_up_regs(i8 %agent1_port_count, ptr %agent1_ports_ptr, i8 4)

    ; Figure out the appropriate rule
    ; index = (agent0_type - L_type) * 5 + agent1_type - E_type
    %code_index.0 = add i8 %agent0_type, -1
    %code_index.1 = mul i8 %code_index.0, 5
    %code_index.2 = add i8 %agent1_type, -4
    %code_index.3 = add i8 %code_index.1, %code_index.2
    %rule_ptr_ptr = getelementptr void ()*, ptr @rule_table, i8 %code_index.3
    %rule_ptr = load void ()*, ptr %rule_ptr_ptr
    
    ; Execute the rule
    call void %rule_ptr()

    ; Free the agents
    call void @free_agent(ptr %agent0_ptr)
    call void @free_agent(ptr %agent1_ptr)

    br label %end

end:
    ; br label %start
    ret void ; DEBUG
}

define i32 @main() {
    ; Set up stacks
    call void @init()

    ; Set up initial tree for "tttt"
    %agent.0 = call ptr @mk_agent(i8 0, ptr @agent_name)
    %agent.1 = call ptr @mk_agent(i8 0, ptr @agent_a)
    %agent.2 = call ptr @mk_agent(i8 0, ptr @agent_l)
    %agent.3 = call ptr @mk_agent(i8 0, ptr @agent_l)
    %agent.4 = call ptr @mk_agent(i8 0, ptr @agent_a)
    call void @load(i8 1, ptr %agent.2)
    call void @load(i8 2, ptr %agent.3)
    call void @push(i8 1, i8 0)
    call void @connect(i8 0, i8 0, i8 2)
    %agent.5 = call ptr @mk_agent(i8 0, ptr @agent_l)
    %agent.6 = call ptr @mk_agent(i8 0, ptr @agent_a)
    call void @load(i8 1, ptr %agent.4)
    call void @load(i8 2, ptr %agent.5)
    call void @connect(i8 1, i8 1, i8 0)
    call void @connect(i8 0, i8 0, i8 2)
    %agent.7 = call ptr @mk_agent(i8 0, ptr @agent_l)
    call void @load(i8 0, ptr %agent.1)
    call void @load(i8 1, ptr %agent.6)
    call void @load(i8 2, ptr %agent.7)
    call void @connect(i8 1, i8 1, i8 0)
    call void @connect(i8 0, i8 0, i8 2)
    call void @load(i8 0, ptr %agent.0)
    call void @load(i8 1, ptr %agent.1)
    call void @connect(i8 1, i8 1, i8 0)

    call void @print_vm(i32 0)
    call void @execute()
    call void @print_vm(i32 1)
    call void @execute()
    call void @print_vm(i32 2)
    call void @execute()
    call void @print_vm(i32 3)
    call void @execute()
    call void @print_vm(i32 4)
    call void @execute()
    call void @print_vm(i32 5)

    ret i32 0
}


; ----------------------------------------------------------------
; ---------------------------- DEBUG -----------------------------
; ----------------------------------------------------------------


@.str = private unnamed_addr constant [4 x i8] c"%d \00", align 1
@.str64 = private unnamed_addr constant [5 x i8] c"%7d \00", align 1
@.str.ln = private unnamed_addr constant [3 x i8] c"\0D\0A\00", align 1
@.str.reg = private unnamed_addr constant [5 x i8] c"REG:\00", align 1
@.str.reg_line = private unnamed_addr constant [7 x i8] c"%d: %d\00", align 1
@.str.heap = private unnamed_addr constant [6 x i8] c"HEAP:\00", align 1
@.str.heap_line = private unnamed_addr constant [21 x i8] c"%d: %d %d %7d %7d | \00", align 1
@.str.pairs = private unnamed_addr constant [7 x i8] c"PAIRS:\00", align 1
@.str.pairs_line = private unnamed_addr constant [8 x i8] c"%d - %d\00", align 1
@.str.sep = private unnamed_addr constant [32 x i8] c"---------- %d -----------------\00", align 1

declare i32 @printf(i8* noundef, ...) #1

define void @print_sep(i32 %index) {
    call void @print_ln()
    call i32 (i8*, ...)
        @printf(i8* noundef getelementptr inbounds
            ([32 x i8], [32 x i8]* @.str.sep, i64 0, i64 0), i32 %index)
    call void @print_ln()
    ret void
}
define void @print_ln() {
    call i32 (i8*, ...)
        @printf(i8* noundef getelementptr inbounds
            ([3 x i8], [3 x i8]* @.str.ln, i64 0, i64 0))
    ret void
}
define void @print_i64(i64 %value) {
    call i32 (i8*, ...)
        @printf(i8* noundef getelementptr inbounds
            ([5 x i8], [5 x i8]* @.str64, i64 0, i64 0),
            i64 noundef %value)
    ret void
}
define void @print_i8(i8 %value) {
    call i32 (i8*, ...)
        @printf(i8* noundef getelementptr inbounds
            ([4 x i8], [4 x i8]* @.str, i64 0, i64 0),
            i8 noundef %value)
    ret void
}
define void @print_i8ptr(i8* %value.ptr) {
    %e = load i8, i8* %value.ptr
    call void @print_i8(i8 %e)
    ret void
}
define void @print_i64ptr(i64* %value.ptr) {
    %e = load i64, i64* %value.ptr
    call void @print_i64(i64 %e)
    ret void
}
define void @print_i8_arr(i8* %array_ptr, i32 %count) {
    ; Create the loop counter
    %counter.ptr = alloca i32
    store i32 0, i32* %counter.ptr
    br label %check

check:
    ; Check if we need to run the loop
    %counter = load i32, i32* %counter.ptr
    %cond = icmp eq i32 %count, %counter
    br i1 %cond, label %end, label %print

print:
    ; Print the nth element
    %e.ptr = getelementptr i8, i8* %array_ptr, i32 %counter
    call void @print_i8ptr(ptr %e.ptr)

    ; Incr the loop counter
    %counter.1 = add i32 %counter, 1
    store i32 %counter.1, i32* %counter.ptr

    br label %check

end:
    ret void
}

define void @print_i64_arr(i64* %array_ptr, i32 %count) {
    ; Create the loop counter
    %counter.ptr = alloca i32
    store i32 0, i32* %counter.ptr
    br label %check

check:
    ; Check if we need to run the loop
    %counter = load i32, i32* %counter.ptr
    %cond = icmp eq i32 %count, %counter
    br i1 %cond, label %end, label %print

print:
    ; Print the nth element
    %e.ptr = getelementptr i64, i64* %array_ptr, i32 %counter
    call void @print_i64ptr(ptr %e.ptr)

    ; Incr the loop counter
    %counter.1 = add i32 %counter, 1
    store i32 %counter.1, i32* %counter.ptr

    br label %check

end:
    ret void
}

define void @print_reg() {
    call i32 (i8*, ...)
        @printf(i8* noundef getelementptr inbounds
            ([5 x i8], [5 x i8]* @.str.reg, i64 0, i64 0))
    call void @print_ln()

    %counter_ptr = alloca i8
    store i8 0, ptr %counter_ptr
    br label %check

check:
    %counter = load i8, ptr %counter_ptr
    %finished = icmp eq i8 %counter, 16
    br i1 %finished, label %end, label %print

print:
    %reg_ptr = getelementptr %Agent*, ptr @reg, i8 %counter
    %value = load ptr, ptr %reg_ptr
    call i32 (i8*, ...)
        @printf(
            i8* noundef getelementptr inbounds ([7 x i8], [7 x i8]* @.str.reg_line, i64 0, i64 0),
            i8 noundef %counter,
            ptr noundef %value)
    call void @print_ln()
    ; Incr counter
    %counter.1 = add i8 1, %counter
    store i8 %counter.1, ptr %counter_ptr
    br label %check

end:
    call void @print_ln()
    ret void
}

define void @print_heap() {
    call i32 (i8*, ...)
        @printf(i8* noundef getelementptr inbounds
            ([6 x i8], [6 x i8]* @.str.heap, i64 0, i64 0))
    call void @print_ln()

    %start_ptr = getelementptr %AgentStack, ptr @agent_stack, i64 0, i32 1
    %end_ptr_ptr = getelementptr %AgentStack, ptr @agent_stack, i64 0, i32 0
    %end_ptr = load ptr, ptr %end_ptr_ptr

    %counter_ptr = alloca i32
    store i32 0, ptr %counter_ptr
    br label %check

check:
    %counter = load i32, ptr %counter_ptr
    %current_ptr = getelementptr %Agent, ptr %start_ptr, i32 %counter
    %finished = icmp eq ptr %current_ptr, %end_ptr
    br i1 %finished, label %end, label %print

print:
    %a = load %Agent, ptr %current_ptr
    %a0 = extractvalue %Agent %a, 0
    %a0.i32 = zext i8 %a0 to i32
    %a1 = extractvalue %Agent %a, 1
    %a2 = extractvalue %Agent %a, 2
    %a3 = extractvalue %Agent %a, 3
    %a4_ptr = getelementptr %Agent, ptr %current_ptr, i64 0, i32 4
    call i32 (i8*, ...)
        @printf(
            i8* noundef getelementptr inbounds ([21 x i8], [21 x i8]* @.str.heap_line, i64 0, i64 0),
            ptr %current_ptr,
            i32 %a0.i32,
            i8 %a1,
            ptr %a2,
            ptr %a3
            )
    call void @print_i64_arr(ptr %a4_ptr, i32 4)
    call void @print_ln()

    %counter.1 = add i32 1, %counter
    store i32 %counter.1, ptr %counter_ptr
    br label %check

end:
    call void @print_ln()
    ret void
}

define void @print_pairs() {
    call i32 (i8*, ...)
        @printf(i8* noundef getelementptr inbounds
            ([7 x i8], [7 x i8]* @.str.pairs, i64 0, i64 0))
    call void @print_ln()

    %start_ptr = getelementptr %PairStack, ptr @pair_stack, i64 0, i32 1
    %end_ptr_ptr = getelementptr %PairStack, ptr @pair_stack, i64 0, i32 0
    %end_ptr = load ptr, ptr %end_ptr_ptr

    %counter_ptr = alloca i32
    store i32 0, ptr %counter_ptr
    br label %check

check:
    %counter = load i32, ptr %counter_ptr
    %current_ptr = getelementptr %Pair, ptr %start_ptr, i32 %counter
    %finished = icmp eq ptr %current_ptr, %end_ptr
    br i1 %finished, label %end, label %print

print:
    %p = load %Pair, ptr %current_ptr
    %p0 = extractvalue %Pair %p, 0
    %p1 = extractvalue %Pair %p, 1
    call i32 (i8*, ...)
        @printf(
            i8* noundef getelementptr inbounds ([8 x i8], [8 x i8]* @.str.pairs_line, i64 0, i64 0),
            ptr %p0,
            ptr %p1
            )
    call void @print_ln()

    %counter.1 = add i32 1, %counter
    store i32 %counter.1, ptr %counter_ptr
    br label %check

end:
    call void @print_ln()
    ret void
}

define void @print_vm(i32 %index) {
    call void @print_reg()
    call void @print_heap()
    call void @print_pairs()
    call void @print_sep(i32 %index)
    ret void
}