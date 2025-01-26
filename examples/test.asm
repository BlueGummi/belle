.ssp [40]       
.sbp [40]       
.start [41]     
int 71          
jmp [176]       
.word 84        
.word 104       
.word 101       
.word 32        
.word 110       
.word 117       
.word 109       
.word 98        
.word 101       
.word 114       
.word 32        
.word 101       
.word 110       
.word 116       
.word 101       
.word 114       
.word 101       
.word 100       
.word 32        
.word 105       
.word 115       
.word 32        
.word 116       
.word 111       
.word 111       
.word 32        
.word 108       
.word 97        
.word 114       
.word 103       
.word 101       
.word 46        
.word 10        
.word 84        
.word 104       
.word 101       
.word 32        
.word 110       
.word 117       
.word 109       
.word 98        
.word 101       
.word 114       
.word 32        
.word 101       
.word 110       
.word 116       
.word 101       
.word 114       
.word 101       
.word 100       
.word 32        
.word 105       
.word 115       
.word 32        
.word 116       
.word 111       
.word 111       
.word 32        
.word 115       
.word 109       
.word 97        
.word 108       
.word 108       
.word 46        
.word 10        
.word 84        
.word 104       
.word 101       
.word 32        
.word 103       
.word 111       
.word 108       
.word 100       
.word 101       
.word 110       
.word 32        
.word 114       
.word 97        
.word 116       
.word 105       
.word 111       
.word 32        
.word 105       
.word 115       
.word 58        
.word 32        
.word 69        
.word 110       
.word 116       
.word 101       
.word 114       
.word 32        
.word 104       
.word 111       
.word 119       
.word 32        
.word 109       
.word 97        
.word 110       
.word 121       
.word 32        
.word 110       
.word 117       
.word 109       
.word 98        
.word 101       
.word 114       
.word 115       
.word 32        
.word 116       
.word 111       
.word 32        
.word 99        
.word 97        
.word 108       
.word 99        
.word 117       
.word 108       
.word 97        
.word 116       
.word 101       
.word 32        
.word 40        
.word 109       
.word 97        
.word 120       
.word 32        
.word 50        
.word 51        
.word 41        
.word 58        
.word 32        
mov r6, 0       
mov r4, 1       
push r6         
push r4         
lea r0, [130]   
lea r1, [176]   
int 8           
int 40          
cmp r0, 1       
bz [219]        
cmp r0, 0       
bz [219]        
cmp r0, 2       
bz [219]        
cmp r0, 24      
bg [223]        
mov r1, 0       
mov r5, 0       
pop r4          
pop r6          
add r5, r4      
add r5, r6      
mov r6, r4      
mov r4, r5      
push r6         
push r4         
cmp r0, r2      
bz [209]        
int 5           
st &r1, r5      
add r1, 1       
add r2, 1       
jmp [193]       
add r1, -2      
mov r7, &r1     
add r1, -1      
mov r6, &r1     
div r7, r6      
lea r0, [109]   
lea r1, [130]   
int 8           
int 7           
hlt             
lea r0, [76]    
lea r1, [109]   
int 8           
hlt             
lea r0, [43]    
lea r1, [76]    
int 8           
hlt             
