// Methods:
//      loadrom 
//      read, write
//      resetPC
//      outerLoop - source addr, dest addr, next instruction addr. Update pc to next inst addr.

// 16 MiB
const MEM_SIZE: usize = 0x1000008;

struct CPU{
    mem: [u8; MEM_SIZE],
    pc: u32, 
}

impl CPU {
    // Implement CPU, all its methods 
   fn new(arg: Type) -> CPU {
       self{

           mem = [0; MEM_SIZE];
           // pc = mem + (mem[2]<<16 | mem[3]<<8 | mem[4]); given in docs
           pc = mem + (mem[2]<<16 | mem[3]<<8 | mem[4]);  
       }
   }
   fn loadRom(arg: Type) -> RetType {

   }
   fn read(arg: Type) -> RetType {
       
   }
   fn write(arg: Type) -> RetType {
      
   }
   fn innerLoop(arg: Type) -> RetType {
      
   }
}

