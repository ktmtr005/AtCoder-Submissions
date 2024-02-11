from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys

# WebDriverの初期化
driver = webdriver.Chrome()  # または他のブラウザに合わせて選択

# 対象のウェブページを開く
url = "https://example.com"  # Aceエディタが埋め込まれているページのURLに変更
driver.get(url)

# Aceエディタの要素を特定するためのセレクタを指定
ace_editor_selector = "div.ace_content"  # これは適切なセレクタに変更する必要があります

# Aceエディタの要素を取得
driver.find_element(By.CSS_SELECTOR, "a.btn-text.toggle-btn-text.submission-code-expand-btn").click()
ace_editor = driver.find_element(By.CSS_SELECTOR, ace_editor_selector)

# Aceエディタのテキストコンテンツを取得
source_code = ace_editor.text
print("取得されたソースコード:")
print(source_code)

# WebDriverを閉じる
driver.quit()
