var sum = 0;
var number = 0;

let is_multiple = fn [x, y] { (x % y) == 0 };

while number < 10 {
  if is_multiple(number, 5) or is_multiple(number, 3) { sum = sum + number };
  number = number + 1
};

sum
