// Loop for 1000 iterations
cp r1 #999
sub r1 #1
Bilz r1 #1
Halt

// Bubble sort
// Key: r1 size of array, r2 index, r3 first value, r4 second value, r5 flag for unsorted pass, r6 temp reg
cp r1 #256 // load size of array into reg 1
cp r2 #0 // load start array position in memory into reg 2
cp r5 #0 // set flag to 0

ld r3 r2 // load first value into r3
add r2 r2 #1 // increment index
ld r4 r2 // load second value into r4
jilt r2 r1 #10 // jump to end if finished array
jilt r3 r4 #12 // jump over swap if already sorted

cp r5 #1 // set flag to 1 - unsuccessful pass
cp r3 r6 // store r3 to temp
cp r3 r4 // swap r3 and r4
cp r4 r6 // store value of r3 back in r4
sub r2 r2 #1 // move back 1 step
st r2 r3 // store swapped first value
add r2 r2 #1 // move forward
st r2 r4 // store swapped second value

add r2 r2 #1 // move to next position
b #-14 // jump back to loading values

jilt #-15 #0 r5 // pass again if previously unsuccessful
halt