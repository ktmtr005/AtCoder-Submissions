def fetchSubmissions() -> int:
    import os
    from fetch_submission import collect_newest_accepted_submissions
    from fetch_submission import get_submission_data

    root = "submissions/"

    submission_data = get_submission_data.getSubmissionData()
    newestSubmits =  collect_newest_accepted_submissions.collectNewestAcceptedSubmissions(submission_data)

    for contestName in newestSubmits:
        path = root + contestName
        os.makedirs(path, exist_ok=True)

    from selenium import webdriver
    from selenium.webdriver.common.by import By
    from time import sleep
    import subprocess

    driver = webdriver.Chrome()

    # 追加したファイルの数を増やす
    add_cnt = 0

    for submissions in newestSubmits.values():
        for sub in submissions:
            # 問題番号の取得
            problem_num = sub["problem_id"]
        
            # 古い問題の場合には数字になっているので、アルファベットに戻す
            if problem_num.isdigit():
                problem_num = chr(int(problem_num)+ord('a')-1)
        
            # 作成するファイルへのパス
            path = root + sub["contest_id"] + "/" + problem_num
            # 拡張子の設定
            # lang_list.jsonから言語と拡張子の対応表を作成
            import json

            lang_data_dir = "lang_list.json"
            with open(lang_data_dir, mode="rt", encoding="utf-8") as f:
                lang_data = json.load(f)
            
            lang = sub["language"]
            try:
                ext = lang_data[lang]
            except KeyError as e:
                # 2023/1 Language Update 以前の提出 (C++, Python Javaのみ)
                if "C++" in lang:
                    path += ".cpp"
                elif "Py" in lang:
                    path += ".py"
                elif "Java" in lang:
                    path += ".java"
            else:
                path += ext

            # 既に提出コードがある場合は取得せず、次の問題の提出を探す
            if os.path.isfile(path): continue
        
            # 提出ページへアクセス
            sub_url = "https://atcoder.jp/contests/" + sub["contest_id"] + "/submissions/" + str(sub["id"])
            driver.get(sub_url)
        
            # 提出コードの取得
            driver.find_element(By.CSS_SELECTOR, "a.btn-text.toggle-btn-text.submission-code-expand-btn").click()
            code = driver.find_element(By.CSS_SELECTOR, "div.ace_content")
            
            # 書き込み
            with open(path, 'w') as f:
                f.write(code.text)
        
            # C++の場合にはclang-formatを使ってフォーマットする
            if "C++" in sub["language"]:
                subprocess.call(["clang-format", "-i",  "-style=file", path])
        
            # 追加したファイルの数を増やす
            add_cnt += 1
            
            # アクセス負荷軽減のために時間をおく(3秒)
            sleep(3)
        
    driver.quit()
    return add_cnt
