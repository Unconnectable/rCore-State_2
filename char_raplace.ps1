# 定义要替换的字符映射表(中文全角 → 英文半角)
$replaceMap = @{
    '(' = '('
    ')' = ')'
    ',' = ','
    '.' = '.'
    ';' = ';'
    ':' = ':'
}

# 初始化计数器
$countStats = @{
    '( → (' = 0
    ') → )' = 0
    ', → ,' = 0
    '. → .' = 0
    '; → ;' = 0
    ': → :' = 0
}

# 获取当前脚本的文件名(不包含路径)
$currentScriptName = Split-Path $MyInvocation.MyCommand.Path -Leaf

# 定义要排除的文件列表(当前脚本和char_replace.sh)
$excludeFiles = @($currentScriptName, 'char_replace.sh')

# 获取当前目录及其子目录中的所有文件,排除指定的文件
$files = Get-ChildItem -Path . -File -Recurse | Where-Object { $excludeFiles -notcontains $_.Name }

foreach ($file in $files) {
    try {
        # 读取文件内容
        $content = Get-Content -Path $file.FullName -Raw
        
        # 替换所有指定的中文标点为英文标点,并统计替换次数
        foreach ($key in $replaceMap.Keys) {
            $escapedKey = [Regex]::Escape($key)
            $oldContent = $content
            $content = $content -replace $escapedKey, $replaceMap[$key]
            
            # 计算替换次数(对比替换前后的差异)
            $oldCount = ([regex]::Matches($oldContent, $escapedKey)).Count
            $newCount = ([regex]::Matches($content, $escapedKey)).Count
            $replacedCount = $oldCount - $newCount
            
            # 更新统计
            $countStats["$key → $($replaceMap[$key])"] += $replacedCount
        }
        
        # 将修改后的内容写回文件
        Set-Content -Path $file.FullName -Value $content -NoNewline
        
        Write-Host "已处理文件: $($file.FullName)"
    }
    catch {
        Write-Host "处理文件 $($file.FullName) 时出错: $_" -ForegroundColor Red
    }
}

# 输出替换统计
Write-Host "`n替换统计:" -ForegroundColor Cyan
foreach ($stat in $countStats.GetEnumerator()) {
    Write-Host "$($stat.Key): $($stat.Value) 次" -ForegroundColor Yellow
}

Write-Host "`n所有文件处理完成!" -ForegroundColor Green