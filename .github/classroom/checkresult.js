/**
 * 解析练习结果JSON文件，生成各练习的评分对象
 * @param {string} outputFile - 练习结果JSON文件的路径（绝对路径或相对路径）
 * @returns {Object} 评分对象（{ 练习名: [得分, 总分] }）或错误信息对象（{ error: 错误描述 }）
 */
function judge(outputFile) {
  // 1. 引入Node.js文件系统模块（内置模块，无需额外安装）
  const fs = require('fs');

  try {
    // 2. 校验入参是否为有效字符串（文件路径）
    if (typeof outputFile !== 'string' || outputFile.trim() === '') {
      return { error: '入参错误：文件路径必须是非空字符串' };
    }
    const filePath = outputFile.trim();

    // 3. 判断文件是否存在（完成TODO）
    if (!fs.existsSync(filePath)) {
      return { error: `文件不存在：${filePath}` };
    }

    // 4. 读取文件内容（处理文件读取异常）
    let fileContent;
    try {
      fileContent = fs.readFileSync(filePath, 'utf8');
    } catch (readErr) {
      return { error: `文件读取失败：${readErr.message}` };
    }

    // 5. 解析JSON（处理JSON格式错误）
    let jsonResult;
    try {
      jsonResult = JSON.parse(fileContent);
    } catch (parseErr) {
      return { error: `JSON格式错误：${parseErr.message}` };
    }

    // 6. 校验解析后的数据结构
    if (typeof jsonResult !== 'object' || jsonResult === null) {
      return { error: 'JSON内容必须是对象' };
    }
    if (!Array.isArray(jsonResult.exercises)) {
      return { error: 'JSON必须包含数组类型的exercises字段' };
    }

    // 7. 生成评分对象（严格校验每个练习项的结构）
    const points = {};
    jsonResult.exercises.forEach((item, index) => {
      // 跳过无效的练习项（不中断整体流程）
      if (
        typeof item !== 'object' ||
        item === null ||
        typeof item.name !== 'string' ||
        item.name.trim() === '' ||
        item.result === undefined
      ) {
        console.warn(`警告：第${index + 1}个练习项格式无效，已跳过`);
        return;
      }
      const exerciseName = item.name.trim();
      // 得分规则：result为真则1/1，否则0/1（避免null/undefined导致的异常）
      points[exerciseName] = Boolean(item.result) ? [1, 1] : [0, 1];
    });

    // 8. 返回最终评分结果（空结果时给出提示）
    return Object.keys(points).length > 0 ? points : { message: '未解析到有效练习项' };

  } catch (globalErr) {
    // 捕获所有未预期的异常
    return { error: `未知错误：${globalErr.message}` };
  }
}
const testFile = './exercise-results.json';



const result = judge(testFile);
console.log('评分结果：', result);
