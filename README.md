# Compiler
## Introduction

This example shows a software development that translates representative programming language statements
into three-address code, an intermediate representation.


## Example

This syntax-directed translator maps code fragments as Fig. 1 into three-address code of the form of Fig. 2.

{ 

    int i; int j; float v; float x; float[100] a;
    
    i = 0 ;
    
    v = 1.5;
    
    j = 1;
    
    while( true ) {
        
        do i = i+1; while( a[i] < v);
        
        do j = j-1; while( a[j] > v);
        
        if( i >= j ) break;
        
        x = a[i]; a[i] = a[j]; a[j] = x;
    
    }

}


Fig 1.




i = 0

v = 1.5

j = 1

L0: iffalse true goto L1

L2: t0 = i + 1

i = t0

t1 = a[i]

if t1 < v goto L2

L3: L4: t2 = j - 1

j = t2

t3 = a[j]

if t3 > v goto L4

L5: iffalse i >= j goto L6

goto L1

L6: t4 = a[i]

x = t4

t5 = a[j]

a[i] = t5

a[j] = x

goto L0

L1: 


Fig 2.



## The Source Language

For specifying the source language syntax, we present the BNF (Backus-Naur Form):

program --> block

block --> { decls stmts }

decls --> decls decl | ϵ

decl --> type **id** ;

type --> type [**num**] | **basic**

stmts --> stmts stmt | ϵ

stmt --> loc = bool;

  | **if** ( bool ) stmt
	     
  | **if** ( bool ) stmt **else** stmt
	     
  | **while** ( bool ) stmt
	     
  | **do** stmt **while** ( bool ) ;
	     
  | **break** ;
	     
  | block
	     
loc --> loc [ bool ] | **id**

bool --> bool || join | join

join --> join && equality | equality

equality --> equality == rel | equality != rel | rel

rel --> expr < expr | expr <= expr | expr >= expr | expr > expr | expr

expr --> expr + term | expr - term | term

term --> term * unary | term / unary | unary

unary -->! unary |- unary | factor

factor --> ( bool ) | loc | **num** | **real** | **true** | **false**


## Three-Address Instructions Code

Three-address code is a sequence of instructions of the form

		x = y op z.

Arrays will be handled by using the following two variants of instructions:

		x [ y ] = z
		
		x = y [ z ]


Three-address instructions are executed in numerical sequence unless forced
to do otherwise by a conditional or unconditional jump. We choose the following
instructions for control flow:

	ifFalse x goto L if x is false, next execute the instruction labeled L
	
	ifTrue x goto L if x is true, next execute the instruction labeled L
	
	goto L next execute the instruction labeled L
	

A label L can be attached to any instruction by prepending a prefix L:. An
instruction can have more than one label.

Finally, we need instructions that copy a value. The following three-address
instruction copies the value of y into x:

		x = y


## Prerequisites
### Install Rust

https://doc.rust-lang.org/book/ch01-01-installation.html#installation

## Execution

### Build front-end

  a) cd <LOCAL_REPOSITORY>/compiler/front-end
  
  b) cargo build

### Edit a file with a source language program

   Example: 
   
    	Edit <LOCAL_REPOSITORY>/compiler/resources/input_program1.txt


### Run front-end

   a) cd <LOCAL_REPOSITORY>/compiler
   
   b) target/build/front-end resources/input_program1.txt
  
 
 
