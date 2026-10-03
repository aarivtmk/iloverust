// function
// js is a loosly typed - python
//
function add(x, y) {
  return x+y
}
let a = 2;
let b = 3;
console.log(`addition of ${a} and ${b} is ${add(a,b)}`)

// anonymous function - a function without a name
// (x, y) => {
//   return x+y
// }

// function expression

let sub = (x, y) => x-y
console.log(sub(1,2))

// function passing function as argument
function print_result(a, b, addition) {
  return (a,b) => a+b;
}
let r = print_result(6,8,add)
console.log("r is ",r)
