let plus = fn[x, y] { x * y };
let ten = fn[f, b] { f(10, b) };

print(ten(plus, 5));

let plus_builder = fn[number] {
  let newfunc = fn[y] { plus(number, y) };
  newfunc
};
# This quite ugly but the currently don't allow anonymous functions :/

let double_of  = plus_builder(2);
double_of(20)
