import { time } from './utils';

export const file = async () => {
  const linreg = await import('wasml-linreg');

  const [fileHandle] = await window.showOpenFilePicker();
  const file = await fileHandle.getFile();

  try {
    const table = await linreg.readCSV(file);
    const transform = new linreg.Transform(table);
    const transformedTable = table.applyTransform(transform);

    const matrix = new linreg.Watrix(transformedTable);

    const [elapsed] = time(() => matrix.shuffle_mut());
    console.log(elapsed);

    const trainingQty = Math.floor(matrix.nrows * 0.7);
    const training = matrix.slice(0, trainingQty);
    const testing = matrix.slice(trainingQty, matrix.nrows);

    console.log(training.data);
    console.log(testing.data);
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
