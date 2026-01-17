const fs = require('fs');
const path = require('path');

// 模拟postinstall.js的环境
const __dirname = 'C:/Users/zuolan/AppData/Roaming/npm/node_modules/@zuolan/micucodeline/scripts';
const platform = 'win32';
const packageName = '@zuolan/micucodeline-win32-x64';
const binaryName = 'micucodeline.exe';

console.log('__dirname:', __dirname);
console.log('packageName:', packageName);
console.log('binaryName:', binaryName);
console.log('');

// 测试路径1: npm/yarn nested
const path1 = path.join(__dirname, '..', 'node_modules', packageName, binaryName);
console.log('路径1 (npm/yarn nested):', path1);
console.log('存在:', fs.existsSync(path1));
console.log('');

// 测试路径2: require.resolve
try {
  const packagePath = require.resolve(packageName + '/package.json');
  const path2 = path.join(path.dirname(packagePath), binaryName);
  console.log('路径2 (require.resolve):', path2);
  console.log('存在:', fs.existsSync(path2));
} catch (e) {
  console.log('路径2 (require.resolve): 失败 -', e.message);
}
console.log('');

// 测试正确的路径
const correctPath = 'C:/Users/zuolan/AppData/Roaming/npm/node_modules/@zuolan/micucodeline/node_modules/@zuolan/micucodeline-win32-x64/micucodeline.exe';
console.log('正确路径:', correctPath);
console.log('存在:', fs.existsSync(correctPath));
