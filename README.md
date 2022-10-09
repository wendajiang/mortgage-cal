## 房贷计算

### Use
```shell
# generate the default toml
mortgage-cal --gen
# change the config by yourself 

# generate the policy empty template, no ahead repay
mortgage-cal -t 

# output the result
mortgage-cal

```
### requirement
1. 公积金贷款额度 + 利率
2. 商贷贷款额度 + 利率
3. 等额本息 还是 等额本金
4. 年限（多少个月）
5. 导入文件每月提前还多少商贷(polisy.csv)
（自己写策略，输出分析结果）（全是0或者默认就是不提前还的结果）
    分为本息和本金的对比
6. 基于 5 的不同策略的对比分析
