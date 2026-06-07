  0:  LD    6,0(0)	* load maxaddress from location 0
  1:  LDC   5,0(0)	* set GP = 0
  2:  ST    0,0(0)	* clear location 0
  3:  IN    0,0,0	* read integer
  4:  ST    0,0(5)	* store to a
  5:  IN    0,0,0	* read integer
  6:  ST    0,1(5)	* store to b
  7:  IN    0,0,0	* read integer
  8:  ST    0,2(5)	* store to c
  9:  LDC   0,0(0)	* load const
 10:  ST    0,0(6)	* op: push left
 11:  LD    0,0(5)	* load id a
 12:  LD    1,0(6)	* op: pop left
 13:  SUB   0,1,0	* relop: left - right
 14:  JLT   0,2(7)	* op <
 15:  LDC   0,0(0)	* relop: false
 16:  LDA   7,1(7)	* relop: skip true
 17:  LDC   0,1(0)	* relop: true
 18:  JEQ   0,13(7)	* if: jump to else
 19:  LD    0,0(5)	* load id a
 20:  ST    0,0(6)	* op: push left
 21:  LD    0,1(5)	* load id b
 22:  LD    1,0(6)	* op: pop left
 23:  MUL   0,1,0	* op *
 24:  ST    0,3(5)	* assign to fact
 25:  LD    0,3(5)	* load id fact
 26:  ST    0,0(6)	* op: push left
 27:  LD    0,2(5)	* load id c
 28:  LD    1,0(6)	* op: pop left
 29:  ADD   0,1,0	* op +
 30:  ST    0,3(5)	* assign to fact
 31:  LDA   7,2(7)	* if: jump to end
 32:  LD    0,2(5)	* load id c
 33:  ST    0,3(5)	* assign to fact
 34:  LD    0,3(5)	* load id fact
 35:  OUT   0,0,0	* write integer
 36:  HALT  0,0,0	* done
