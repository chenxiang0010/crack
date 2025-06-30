# Release Script for Crack Project
# 用于快速创建和推送版本标签的PowerShell脚本

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,
    
    [Parameter(Mandatory=$false)]
    [string]$Message = "",
    
    [Parameter(Mandatory=$false)]
    [switch]$DryRun = $false,
    
    [Parameter(Mandatory=$false)]
    [switch]$Force = $false
)

# 颜色输出函数
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    
    $colors = @{
        "Red" = [ConsoleColor]::Red
        "Green" = [ConsoleColor]::Green
        "Yellow" = [ConsoleColor]::Yellow
        "Blue" = [ConsoleColor]::Blue
        "Cyan" = [ConsoleColor]::Cyan
        "White" = [ConsoleColor]::White
    }
    
    Write-Host $Message -ForegroundColor $colors[$Color]
}

# 验证版本号格式
function Test-VersionFormat {
    param([string]$Version)
    
    # 支持的版本格式：v1.0.0, v1.0.0-beta, v1.0.0-alpha.1 等
    $pattern = '^v\d+\.\d+\.\d+(-[a-zA-Z0-9\.-]+)?$'
    return $Version -match $pattern
}

# 检查Git状态
function Test-GitStatus {
    $status = git status --porcelain
    if ($status) {
        Write-ColorOutput "❌ 工作目录不干净，请先提交或暂存更改：" "Red"
        git status --short | Out-Host
        return $false
    }
    return $true
}

# 检查标签是否已存在
function Test-TagExists {
    param([string]$Tag)
    
    $existingTag = git tag -l $Tag
    if ($existingTag) {
        Write-ColorOutput "❌ 标签 '$Tag' 已存在" "Red"
        return $true
    }
    return $false
}

# 运行测试
function Invoke-Tests {
    Write-ColorOutput "🧪 运行测试..." "Blue"

    cargo test | Out-Host
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "❌ 测试失败" "Red"
        return $false
    }

    Write-ColorOutput "✅ 测试通过" "Green"
    return $true
}

# 构建Release版本
function Invoke-Build {
    Write-ColorOutput "🔨 构建Release版本..." "Blue"

    cargo build --release | Out-Host
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "❌ 构建失败" "Red"
        return $false
    }

    Write-ColorOutput "✅ 构建成功" "Green"
    return $true
}

# 创建标签
function New-GitTag {
    param(
        [string]$Tag,
        [string]$Message
    )
    
    if ([string]::IsNullOrEmpty($Message)) {
        $Message = "Release $Tag"
    }
    
    Write-ColorOutput "🏷️ 创建标签 '$Tag'..." "Blue"
    
    if ($DryRun) {
        Write-ColorOutput "🔍 [DRY RUN] 将创建标签: git tag -a $Tag -m `"$Message`"" "Yellow"
        return $true
    }
    
    git tag -a $Tag -m $Message
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "❌ 创建标签失败" "Red"
        return $false
    }
    
    Write-ColorOutput "✅ 标签创建成功" "Green"
    return $true
}

# 推送标签
function Push-GitTag {
    param([string]$Tag)
    
    Write-ColorOutput "📤 推送标签到远程仓库..." "Blue"
    
    if ($DryRun) {
        Write-ColorOutput "🔍 [DRY RUN] 将推送标签: git push origin $Tag" "Yellow"
        return $true
    }
    
    git push origin $Tag
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "❌ 推送标签失败" "Red"
        return $false
    }
    
    Write-ColorOutput "✅ 标签推送成功" "Green"
    return $true
}

# 主函数
function Main {
    Write-ColorOutput "🚀 开始发布流程..." "Cyan"
    Write-ColorOutput "版本: $Version" "White"
    
    # 验证版本号格式
    if (-not (Test-VersionFormat $Version)) {
        Write-ColorOutput "❌ 版本号格式无效。请使用格式：v1.0.0 或 v1.0.0-beta" "Red"
        exit 1
    }
    
    # 检查标签是否已存在
    if ((Test-TagExists $Version) -and -not $Force) {
        Write-ColorOutput "💡 使用 -Force 参数可以强制覆盖现有标签" "Yellow"
        exit 1
    }
    
    # 检查Git状态
    if (-not (Test-GitStatus)) {
        Write-ColorOutput "💡 请先提交或暂存所有更改后再发布" "Yellow"
        exit 1
    }
    
    # 运行测试
    if (-not (Invoke-Tests)) {
        exit 1
    }
    
    # 构建Release版本
    if (-not (Invoke-Build)) {
        exit 1
    }
    
    # 如果标签已存在且使用Force参数，先删除旧标签
    if ((Test-TagExists $Version) -and $Force) {
        Write-ColorOutput "🗑️ 删除现有标签..." "Yellow"
        if (-not $DryRun) {
            git tag -d $Version
            git push origin :refs/tags/$Version 2>$null
        }
    }
    
    # 创建标签
    if (-not (New-GitTag $Version $Message)) {
        exit 1
    }
    
    # 推送标签
    if (-not (Push-GitTag $Version)) {
        exit 1
    }
    
    if ($DryRun) {
        Write-ColorOutput "🔍 DRY RUN 完成 - 没有实际执行任何操作" "Yellow"
    } else {
        Write-ColorOutput "🎉 发布流程完成！" "Green"
        Write-ColorOutput "📋 接下来的步骤：" "Cyan"
        Write-ColorOutput "1. 访问 GitHub Actions 页面查看构建进度" "White"
        Write-ColorOutput "2. 构建完成后检查 GitHub Releases 页面" "White"
        Write-ColorOutput "3. 验证发布文件是否正确" "White"
        
        $repoUrl = git config --get remote.origin.url
        if ($repoUrl -match "github\.com[:/](.+?)(?:\.git)?$") {
            $repoPath = $matches[1]
            Write-ColorOutput "🔗 GitHub Actions: https://github.com/$repoPath/actions" "Blue"
            Write-ColorOutput "🔗 Releases: https://github.com/$repoPath/releases" "Blue"
        }
    }
}

# 显示帮助信息
function Show-Help {
    Write-ColorOutput "Crack 项目发布脚本" "Cyan"
    Write-ColorOutput ""
    Write-ColorOutput "用法:" "Yellow"
    Write-ColorOutput "  .\scripts\release.ps1 -Version v1.0.0 [-Message `"发布说明`"] [-DryRun] [-Force]" "White"
    Write-ColorOutput ""
    Write-ColorOutput "参数:" "Yellow"
    Write-ColorOutput "  -Version    版本号 (必需，格式: v1.0.0)" "White"
    Write-ColorOutput "  -Message    标签消息 (可选)" "White"
    Write-ColorOutput "  -DryRun     预览模式，不执行实际操作" "White"
    Write-ColorOutput "  -Force      强制覆盖现有标签" "White"
    Write-ColorOutput ""
    Write-ColorOutput "示例:" "Yellow"
    Write-ColorOutput "  .\scripts\release.ps1 -Version v1.0.0" "White"
    Write-ColorOutput "  .\scripts\release.ps1 -Version v1.1.0 -Message `"添加新功能`"" "White"
    Write-ColorOutput "  .\scripts\release.ps1 -Version v1.0.1 -DryRun" "White"
}

# 检查是否需要显示帮助
if ($Version -eq "help" -or $Version -eq "-h" -or $Version -eq "--help") {
    Show-Help
    exit 0
}

# 执行主函数
try {
    Main
} catch {
    Write-ColorOutput "❌ 发生错误: $($_.Exception.Message)" "Red"
    exit 1
}
