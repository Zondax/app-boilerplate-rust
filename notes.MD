if arm-none-eabi-gcc is not installed. An error message would be useful.

brew install gcc gcc-arm-none-eabi clang
brew install llvm
echo 'export PATH="/opt/homebrew/opt/llvm/bin:$PATH"' >> /Users/lenij/.zshrc
