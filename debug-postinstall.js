const fs = require('fs');
const path = require('path');
const os = require('os');

console.log('=== 调试信息 ===');
console.log('__dirname:', __dirname);
console.log('process.cwd():', process.cwd());
console.log('');

const platform = process.platform;
const arch = process.arch;
const homeDir = os.homedir();
const claudeDir = path.join(homeDir, '.claude', 'micucodeline');

console.log('Platform:', platform);
console.log('Arch:', arch);
console.log('Home Dir:', homeDir);
console.log('Claude Dir:', claudeDir);
console.log('');

const platformKey = `${platform}-${arch}`;
const packageMap = {
  'darwin-x64': '@zuolan/micucodeline-darwin-x64',
  'darwin-arm64': '@zuolan/micucodeline-darwin-arm64',
  'linux-x64': '@zuolan/micucodeline-linux-x64',
  'linux-x64-musl': '@zuolan/micucodeline-linux-x64-musl',
  'win32-x64': '@zuolan/micucodeline-win32-x64',
  'win32-ia32': '@zuolan/micucodeline-win32-x64',
};

const packageName = packageMap[platformKey];
const binaryName = platform === 'win32' ? 'micucodeline.exe' : 'micucodeline';

console.log('Platform Key:', platformKey);
console.log('Package Name:', packageName);
console.log('Binary Name:', binaryName);
console.log('');

// 测试所有可能的路径
const possiblePaths = [
  path.join(__dirname, '..', 'node_modules', packageName, binaryName),
  (() => {
    try {
      const packagePath = require.resolve(packageName + '/package.json');
      return path.join(path.dirname(packagePath), binaryName);
    } catch {
      return null;
    }
  })(),
];

console.log('=== 测试路径 ===');
possiblePaths.forEach((p, i) => {
  if (p) {
    console.log(`路径${i + 1}:`, p);
    console.log(`存在:`, fs.existsSync(p));
  } else {
    console.log(`路径${i + 1}: null`);
  }
});
