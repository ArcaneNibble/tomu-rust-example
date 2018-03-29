MEMORY
{
  FLASH : ORIGIN = 0x00004000, LENGTH = 0xC000
  RAM : ORIGIN = 0x20000000, LENGTH = 8K
}

/* Size of the heap (in bytes) */
_heap_size = 1K;
