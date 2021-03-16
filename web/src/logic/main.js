export const file = async () => {
  const linreg = await import('wasml-linreg');

  const [fileHandle] = await window.showOpenFilePicker();
  const file = await fileHandle.getFile();

  try {
    const table = await linreg.newTable(file);
    console.log(table.headers);
  } catch (err) {
    console.error(err);
  }
};
