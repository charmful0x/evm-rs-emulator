use super::utils::errors::ExecutionError;
use std::ptr;

#[derive(Debug)]
pub struct Memory {
    pub heap: Vec<u8>,
}

impl Memory {
    pub fn new(data: Option<Vec<u8>>) -> Self {
        Self {
            heap: if data.is_some() {
                data.unwrap()
            } else {
                vec![0; 0]
            },
        }
    }

    // Extend memory by size
    pub fn extend(&mut self, size: usize) {
        self.heap.extend(vec![0; size]);
    }

    // Load bytes from memory
    pub unsafe fn read(&self, address: usize, size: usize) -> Result<Vec<u8>, ExecutionError> {
        if address + size > self.heap.len() {
            return Err(ExecutionError::OutOfBoundsMemory);
        }

        let ptr = self.heap.as_ptr().add(address);
        let mut data = vec![0; size];
        ptr::copy(ptr, data.as_mut_ptr(), size);

        Ok(data)
    }

    // Load bytes from memory
    pub unsafe fn write(&mut self, address: usize, data: Vec<u8>) -> Result<(), ExecutionError> {
        // check if memory should be extended
        if address + data.len() > self.heap.len() {
            self.extend(address + data.len() - self.heap.len());
        }

        let ptr = self.heap.as_mut_ptr().add(address);
        ptr::copy(data.as_ptr(), ptr, data.len());

        Ok(())
    }

    // Load 32 bytes from memory
    pub unsafe fn mload(&self, address: usize) -> Result<[u8; 32], ExecutionError> {
        if address + 32 > self.heap.len() {
            return Err(ExecutionError::OutOfBoundsMemory);
        }

        let ptr = self.heap.as_ptr().add(address);
        Ok(ptr::read(ptr as *const [u8; 32]))
    }

    // Store 32 bytes in memory
    pub unsafe fn mstore(&mut self, address: usize, data: [u8; 32]) -> Result<(), ExecutionError> {
        // Check if memory should be extended
        if address + 32 > self.heap.len() {
            self.extend(address + 32 - self.heap.len());
        }

        let ptr = self.heap.as_mut_ptr().add(address);
        ptr::write(ptr as *mut [u8; 32], data);

        Ok(())
    }

    // Get the memory size
    pub fn msize(&self) -> usize {
        self.heap.len()
    }
}
