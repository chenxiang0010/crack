# GitHub CI 自动发布配置完成

## 📋 已创建的文件

### 1. GitHub Actions 工作流
- **文件**: `.github/workflows/release.yml`
- **功能**: 自动构建和发布Windows版本

### 2. 发布脚本
- **PowerShell版本**: `scripts/release.ps1` (Windows)
- **Bash版本**: `scripts/release.sh` (Linux/macOS)

### 3. 文档
- **发布指南**: `RELEASE_GUIDE.md`
- **CI配置说明**: `CI_SETUP.md` (本文件)

## 🚀 CI工作流特性

### 触发条件
- 推送以 `v` 开头的标签时自动触发
- 例如：`v1.0.0`、`v2.1.3`、`v1.0.0-beta`

### 构建流程
1. **环境准备**
   - Windows最新版本运行器
   - 安装Rust稳定版工具链
   - 配置Rust缓存加速构建

2. **编译构建**
   - 执行 `cargo build --release`
   - 生成优化的可执行文件

3. **文件压缩**
   - 下载UPX v4.2.2压缩工具
   - 使用 `--best --lzma` 参数压缩可执行文件
   - 通常可减小50-70%的文件大小

4. **文件准备**
   - 重命名为 `crack-windows-x64.exe`
   - 复制配置文件示例
   - 生成使用说明文档

5. **自动发布**
   - 创建GitHub Release
   - 上传构建文件
   - 生成详细的发布说明

### 发布内容
每次发布包含：
- `crack-windows-x64.exe` - 压缩后的可执行文件
- `config-example.json` - 配置文件示例
- `README.txt` - 使用说明

## 🛠️ 使用方法

### 方法1: 使用发布脚本 (推荐)

**Windows (PowerShell):**
```powershell
# 基本发布
.\scripts\release.ps1 -Version v1.0.0

# 带发布说明
.\scripts\release.ps1 -Version v1.0.0 -Message "修复重要bug"

# 预览模式（不实际执行）
.\scripts\release.ps1 -Version v1.0.0 -DryRun

# 强制覆盖现有标签
.\scripts\release.ps1 -Version v1.0.0 -Force
```

### 方法2: 手动创建标签

```bash
# 创建标签
git tag v1.0.0

# 推送标签
git push origin v1.0.0
```

## 📊 脚本功能对比

| 功能 | PowerShell脚本 | Bash脚本 | 手动操作 |
|------|----------------|----------|----------|
| 版本格式验证 | ✅ | ✅ | ❌ |
| Git状态检查 | ✅ | ✅ | ❌ |
| 自动测试 | ✅ | ✅ | ❌ |
| 自动构建 | ✅ | ✅ | ❌ |
| 预览模式 | ✅ | ✅ | ❌ |
| 彩色输出 | ✅ | ✅ | ❌ |
| 错误处理 | ✅ | ✅ | ❌ |

## 🔧 CI配置详解

### 环境变量
```yaml
env:
  CARGO_TERM_COLOR: always  # 启用彩色输出
```

### 关键步骤

1. **Rust工具链设置**
   ```yaml
   - uses: dtolnay/rust-toolchain@stable
     with:
       toolchain: stable
   ```

2. **缓存配置**
   ```yaml
   - uses: Swatinem/rust-cache@v2
     with:
       cache-on-failure: true
   ```

3. **UPX压缩**
   ```yaml
   - name: 使用UPX压缩可执行文件
     run: upx --best --lzma target/release/crack.exe
   ```

4. **Release创建**
   ```yaml
   - uses: softprops/action-gh-release@v1
     with:
       files: |
         release/crack-windows-x64.exe
         release/config-example.json
         release/README.txt
   ```

## 📈 优化特性

### 构建优化
- **Rust缓存**: 加速重复构建
- **并行编译**: 利用多核CPU
- **Release模式**: 最大优化级别

### 文件优化
- **UPX压缩**: 显著减小文件大小
- **LZMA算法**: 最佳压缩比
- **保持兼容性**: 压缩后仍可正常运行

### 用户体验
- **中文界面**: 友好的中文提示
- **详细日志**: 完整的构建信息
- **自动化**: 一键发布流程

## 🔍 监控和调试

### 查看构建状态
1. 访问仓库的 **Actions** 标签页
2. 找到 **Release** 工作流
3. 查看具体的构建日志

### 常见问题排查

**构建失败:**
- 检查代码是否能本地编译
- 查看Actions日志中的错误信息
- 确认依赖项是否正确

**UPX压缩失败:**
- 网络问题导致UPX下载失败
- 可以重新触发工作流

**Release创建失败:**
- 检查仓库权限设置
- 确认GITHUB_TOKEN有足够权限

## 📝 版本号规范

建议使用语义化版本号：

- `v1.0.0` - 主要版本（破坏性更改）
- `v1.1.0` - 次要版本（新功能）
- `v1.0.1` - 补丁版本（bug修复）
- `v1.0.0-beta` - 预发布版本
- `v1.0.0-alpha.1` - 内测版本

## 🎯 下一步操作

1. **测试CI流程**
   ```bash
   # 创建测试标签
   git tag v0.1.0-test
   git push origin v0.1.0-test
   ```

2. **监控构建**
   - 访问GitHub Actions页面
   - 查看构建进度和日志

3. **验证发布**
   - 检查GitHub Releases页面
   - 下载并测试生成的文件

4. **正式发布**
   ```bash
   # 使用脚本发布正式版本
   .\scripts\release.ps1 -Version v1.0.0
   ```

## 🆘 获取帮助

如果遇到问题：

1. **查看文档**: `RELEASE_GUIDE.md`
2. **检查日志**: GitHub Actions构建日志
3. **脚本帮助**: 
   - `.\scripts\release.ps1 -Version help`
   - `./scripts/release.sh --help`

配置已完成，现在可以开始使用自动化发布流程了！🎉
