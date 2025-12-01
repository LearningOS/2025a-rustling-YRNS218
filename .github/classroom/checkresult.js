// 引入Node.js内置文件系统模块（无需额外安装依赖）
const fs = require('fs');

/**
 * 解析练习结果JSON文件，生成各练习的评分对象
 * @param {string} outputFile - 练习结果JSON文件的路径（绝对路径或相对路径）
 * @returns {Object} 成功返回评分对象 { 练习名: [得分, 总分] }；失败返回 { error: 错误描述 }
 */
function judge(outputFile) {
  try {
    // 1. 校验入参有效性
    if (typeof outputFile !== 'string' || outputFile.trim() === '') {
      return { error: '文件路径必须是 non-empty 字符串' };
    }
    const filePath = outputFile.trim();

    // 2. 检查文件是否存在（完成原代码TODO）
    if (!fs.existsSync(filePath)) {
      return { error: `文件不存在：${filePath}` };
    }

    // 3. 读取文件内容（处理读取异常）
    let fileContent;
    try {
      fileContent = fs.readFileSync(filePath, 'utf8');
    } catch (readErr) {
      return { error: `文件读取失败：${readErr.message}` };
    }

    // 4. 校验文件内容非空
    if (!fileContent.trim()) {
      return { error: '文件内容为空，无法解析JSON' };
    }

    // 5. 解析JSON内容（处理JSON格式错误）
    let jsonResult;
    try {
      jsonResult = JSON.parse(fileContent);
    } catch (parseErr) {
      return { error: `JSON格式错误：${parseErr.message}（行号：${parseErr.lineNumber}）` };
    }

    // 6. 校验JSON结构符合预期
    if (typeof jsonResult !== 'object' || jsonResult === null) {
      return { error: 'JSON内容必须是对象类型' };
    }
    if (!Array.isArray(jsonResult.exercises)) {
      return { error: 'JSON必须包含 "exercises" 字段（数组类型）' };
    }

    // 7. 生成评分结果（严格校验每个练习项）
    const points = {};
    jsonResult.exercises.forEach((item, index) => {
      // 跳过无效练习项，不中断整体流程
      if (
        typeof item !== 'object' ||
        item === null ||
        typeof item.name !== 'string' ||
        item.name.trim() === '' ||
        item.result === undefined
      ) {
        console.warn(`⚠️  跳过无效练习项（索引${index}）：缺少name或result字段`);
        return;
      }

      const exerciseName = item.name.trim();
      // 评分规则：result为真 → [1,1]，否则 → [0,1]（Boolean()统一转换避免异常）
      points[exerciseName] = Boolean(item.result) ? [1, 1] : [0, 1];
    });

    // 8. 处理无有效练习项的情况
    return Object.keys(points).length > 0 
      ? points 
      : { error: '未解析到有效练习项（exercises数组为空或所有项格式无效）' };

  } catch (globalErr) {
    // 捕获所有未预期的异常，返回明确错误信息
    return { error: `未知错误：${globalErr.message}` };
  }
}

// 导出函数（供其他模块调用，若无需导出可删除）
module.exports = { judge };

// ------------------------------
// 本地测试示例（运行代码时自动执行，可根据需求删除）
// ------------------------------
if (require.main === module) {
  // 测试用JSON文件路径（需提前创建该文件，内容参考下方示例）
  const testFilePath = './exercise-results.json';

  // 示例JSON文件内容（exercise-results.json）：
  // {
  //   "exercises": [
  //     { "name": "变量声明", "result": true },
  //     { "name": "函数定义", "result": false },
  //     { "name": "Promise异步", "result": true },
  //     { "name": "数组方法", "result": null } // 会被转为false，评分[0,1]
  //   ]
  // }

  const result = judge(testFilePath);
  console.log('📊 评分结果：', JSON.stringify(result, null, 2));
}
