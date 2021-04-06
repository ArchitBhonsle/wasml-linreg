export const time = callback => {
  const before = new Date();
  const returned = callback();
  const after = new Date();

  return [after - before, returned];
};
