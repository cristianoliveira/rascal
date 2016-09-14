begin
  imut x = 2;
  mut y = 0;

  while y < 10 begin
    y = x + y
  end;

  return y == 10
end
