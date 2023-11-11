## virtual machine
1. How a virtual machine works?
A virtual machine first parse high - level language source code into an AST tree for analisys before transforming it further into a series of pre - coded instructions known as Intermediate Representation (IR). The execution environment will then read these instructions for execution.

2. Pre - coded instructions (Op code)
An Intel cpu has more than 100 OpCodes. For our calculator we only need to have around 8 OpCodes.

The size of our OpCode will be 8 bit. Therefore, data will be converted into array of u8.

* OpConstant
* OpPop         // pop is needed for execution
* OpAdd
* OpSub
* OpMul
* OpDiv
* OpPlus
* OpMinus

3. Virtual machine
A virtual machine is created to handle bytecode by iteratively going through each bytes and then processing them. Temporary result is stored in a stack for further processing.