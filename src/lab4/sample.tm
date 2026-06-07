  0:  LD    6,0(0)	* load maxaddress from location 0
  1:  LDC   5,0(0)	* set GP = 0
  2:  ST    0,0(0)	* clear location 0
  3:  IN    0,0,0	* read integer
  4:  ST    0,0(5)	* store to x
  5:  LDC   0,0(0)	* load const
  6:  ST    0,0(6)	* op: push left
  7:  LD    0,0(5)	* load id x
  8:  LD    1,0(6)	* op: pop left
  9:  SUB   0,1,0	* relop: left - right
 10:  JLT   0,2(7)	* op <
 11:  LDC   0,0(0)	* relop: false
 12:  LDA   7,1(7)	* relop: skip true
 13:  LDC   0,1(0)	* relop: true
 14:  JEQ   0,26(7)	* if: jump to end
 15:  LDC   0,1(0)	* load const
 16:  ST    0,1(5)	* assign to fact
 17:  LD    0,1(5)	* load id fact
 18:  ST    0,0(6)	* op: push left
 19:  LD    0,0(5)	* load id x
 20:  LD    1,0(6)	* op: pop left
 21:  MUL   0,1,0	* op *
 22:  ST    0,1(5)	* assign to fact
 23:  LD    0,0(5)	* load id x
 24:  ST    0,0(6)	* op: push left
 25:  LDC   0,1(0)	* load const
 26:  LD    1,0(6)	* op: pop left
 27:  SUB   0,1,0	* op -
 28:  ST    0,0(5)	* assign to x
 29:  LD    0,0(5)	* load id x
 30:  ST    0,0(6)	* op: push left
 31:  LDC   0,0(0)	* load const
 32:  LD    1,0(6)	* op: pop left
 33:  SUB   0,1,0	* relop: left - right
 34:  JEQ   0,2(7)	* op =
 35:  LDC   0,0(0)	* relop: false
 36:  LDA   7,1(7)	* relop: skip true
 37:  LDC   0,1(0)	* relop: true
 38:  JEQ   0,-22(7)	* repeat: loop back if false
 39:  LD    0,1(5)	* load id fact
 40:  OUT   0,0,0	* write integer
 41:  HALT  0,0,0	* done
