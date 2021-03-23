export const file = async () => {
  const linreg = await import('wasml-linreg');

  const [fileHandle] = await window.showOpenFilePicker();
  const file = await fileHandle.getFile();

  try {
    const table = await linreg.readCSV(file);
    console.log(table.data);
    const transform = new linreg.Transform(table);
    const tTable = table.applyTransform(transform);
    console.log(tTable.data);
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
