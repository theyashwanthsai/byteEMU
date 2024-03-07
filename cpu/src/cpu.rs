// Methods:
//      loadrom 
//      read, write
//      resetPC
//      outerLoop - source addr, dest addr, next instruction addr. Update pc to next inst addr.

// 16 MiB
use std::io;
use std::fs;
const MEM_SIZE: usize = 0x1000008;

struct CPU{
    mem: [u8; MEM_SIZE],
    pc: u32, 
}

impl CPU {
    // Implement CPU, all its methods 
   fn new() -> CPU {
       let mut cpu = CPU{
           mem: [0; MEM_SIZE],
           // pc = mem + (mem[2]<<16 | mem[3]<<8 | mem[4]); given in docs
           pc: mem + (mem[2]<<16 | mem[3]<<8 | mem[4]), 
       };
        return cpu;
   }
  
   // Load rom a given location
   fn loadRom(&self, loc: &str) {
       let mut rom = fs::read(loc)?;
       if rom.len() <= MEM_SIZE{
            for i in 0..rom.len(){
                self.mem[i] = rom[i];
            }
        }
        else{
            panic!("Rom is bigger than 16MiB");
        }
   }

   // Read data from given location in memory
   fn read(&self, loc: u8) -> u8 {
        if(loc < MEM_SIZE){
           return mem[loc];
        }else{
            panic!("Out of bound error!");
        }
   }

   // Write to memory
   fn write(&mut self, loc: u8, data: u8) {
       if(loc < MEM_SIZE){
           mem[loc] = data;
       }else{
            panic!("Out of bound error!");
        }
   }

   fn innerLoop(arg: Type) -> RetType {
     A = ;
     B = ;
     C = ;

     self.write(B, self.read(A));
     self.pc = C;
            
   }
}

