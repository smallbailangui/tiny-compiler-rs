  0:  LD    6,0(0)	* load maxaddress from location 0
  1:  LDC   5,0(0)	* set GP = 0
  2:  ST    0,0(0)	* clear location 0
  3:  IN    0,0,0	* read integer
  4:  ST    0,0(5)	* store to a
  5:  IN    0,0,0	* read integer
  6:  ST    0,1(5)	* store to b
  7:  LD    0,0(5)	* load id a
  8:  ST    0,0(6)	* op: push left
  9:  LD    0,1(5)	* load id b
 10:  LD    1,0(6)	* op: pop left
 11:  ADD   0,1,0	* op +
 12:  ST    0,2(5)	* assign to sum
 13:  LD    0,2(5)	* load id sum
 14:  OUT   0,0,0	* write integer
 15:  HALT  0,0,0	* done
