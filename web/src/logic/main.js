export const file = async () => {
  const linreg = await import('wasml-linreg');

  const [fileHandle] = await window.showOpenFilePicker();
  const file = await fileHandle.getFile();

  try {
    const table = await linreg.readCSV(file);
    const transform = new linreg.Transform(table);
    const tTable = table.applyTransform(transform);
    console.log(tTable.data);
    const matrix = new linreg.Watrix(tTable);
    console.log(matrix.data);
    console.log(matrix.height, matrix.width);
  } catch (err) {
    console.error(err);
  }
};

export const run = async () => {
  try {
  } catch (err) {
    console.error(err);
  }
};
