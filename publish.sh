#!/bin/bash

echo "🚀 Preparing npm package for publishing..."

# 빌드 실행
echo "📦 Building package..."
./build.sh

# pkg 디렉토리로 이동
cd pkg

# npm 로그인 확인
echo "🔐 Checking npm login status..."
if ! npm whoami > /dev/null 2>&1; then
    echo "❌ Not logged in to npm. Please run 'npm login' first."
    exit 1
fi

# 패키지 이름 중복 확인
echo "🔍 Checking package name availability..."
PACKAGE_NAME=$(node -p "require('./package.json').name")
if npm view "$PACKAGE_NAME" version > /dev/null 2>&1; then
    echo "⚠️  Package '$PACKAGE_NAME' already exists on npm."
    echo "Current version: $(npm view "$PACKAGE_NAME" version)"
    echo "New version: $(node -p "require('./package.json').version")"
    read -p "Do you want to continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Publishing cancelled."
        exit 1
    fi
fi

# 패키지 검증
echo "✅ Validating package..."
npm pack --dry-run

# 실제 배포
echo "🚀 Publishing to npm..."
read -p "Are you ready to publish? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    npm publish
    echo "✅ Package published successfully!"
    echo "📦 Package URL: https://www.npmjs.com/package/$PACKAGE_NAME"
else
    echo "❌ Publishing cancelled."
    exit 1
fi
