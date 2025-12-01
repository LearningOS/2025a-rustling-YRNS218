const fs = require('fs'); 
function judge(outputFile) {
  try {
    if (typeof outputFile !== 'string' || outputFile.trim() === '') {
      throw new Error('文件路径必须为非空字符串');
    }
    const filePath = outputFile.trim();
    if (!fs.existsSync(filePath)) {
      throw new Error(`文件不存在: ${filePath}`);
    }
    const fileContent = fs.readFileSync(filePath, 'utf8');
    if (!fileContent) {
      throw new Error('文件内容为空');
    }
    const jsonResult = JSON.parse(fileContent);

    if (!jsonResult || typeof jsonResult !== 'object' || !Array.isArray(jsonResult.exercises)) {
      throw new Error('JSON格式错误，缺少exercises数组');
    }
    const points = {};
    jsonResult.exercises.forEach((item, index) => {
      if (!item || typeof item !== 'object' || !item.name || item.result === undefined) {
        throw new Error(`exercises第${index + 1}项格式错误，缺少name或result`);
      }
      points[item.name] = item.result ? [1, 1] : [0, 1];
    });

    return points;

  } catch (e) {
    console.error('处理失败:', e.message);
    return { error: e.message }; 
  }
}

module.exports.judge = judge;
