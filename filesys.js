const fs = require('fs/promises');
const path = require('path');

/**
 *
 * @param {string} folderName
 * @param {number} from
 * @returns {Promise<*[]>}
 */
async function findDirFiles(folderName, from) {
  // this array will hold sales files as they are found
  let salesFiles = [];

  async function findFiles(folderName) {
    // read all the items in the current folder
    const items = await fs.readdir(folderName, { withFileTypes: true });

    // iterate over each found item
    for (const item of items) {
      // if the item is a directory, it will need to be searched
      if (item.isDirectory()) {
        // call this method recursively, appending the folder name to make a new path
        await findFiles(path.join(folderName, item.name));
      } else {
        const fullPath = path.join(folderName, item.name).substr(from);
        // store the file path in the salesFiles array
        const stats = await fs.stat(fullPath);
        // size in bytes convert to
        const size = stats.size / (1024 * 1024);

        if (size > 0.9) {
          salesFiles.push(`${fullPath} ${size}`);
        }

        // salesFiles.push(`${fullPath} ${size}`);
      }
    }
  }

  await findFiles(folderName);

  return salesFiles;
}

async function main() {
  const baseFolder = 'tmp/node_modules';
  const p = path.resolve(__dirname, baseFolder);
  console.log('base folder: ', p);
  // const salesFiles = await findDirFiles(path.join(__dirname, "tmp"));
  const salesFiles = await findDirFiles(p, p.length - baseFolder.length);
  // console.log(salesFiles);
  for (const salesFile of salesFiles) {
    console.log(salesFile);
  }
  console.log(salesFiles.length)
}

main().catch((error) => {
  console.log(error);
});
