begin
  mut sum = 0;
  mut number = 0;

  fn is_multiple = [x, y] { (x % y) == 0 };

  while number < 10 {
    if is_multiple(number, 5) or is_multiple(number, 3) { sum = sum + number };

    number = number + 1
  };

  sum
end
