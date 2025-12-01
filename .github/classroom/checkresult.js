function judge(outputFile) {
  try {
    // 补充文件存在性判断（Node.js环境示例）
    const fs = require('fs');
    if (!fs.existsSync(outputFile)) {
      return { error: '文件不存在' };
    }
    // 读取文件内容（原代码可能遗漏了读取步骤）
    const fileContent = fs.readFileSync(outputFile, 'utf8');
    const jsonResult = JSON.parse(fileContent);
    
    // 校验exercises字段
    if (!Array.isArray(jsonResult.exercises)) {
      return { error: 'exercises字段格式错误' };
    }

    const points = {};
    jsonResult.exercises.forEach((item) => {
      // 校验item结构
      if (!item.name || item.result === undefined) return;
      points[item.name] = item.result ? [1, 1] : [0, 1];
    });
    return points;
  } catch (e) {
    return { error: e.message };
  }
}
