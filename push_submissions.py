def pushSubmission(add_cnt: int):
    if add_cnt == 0:
        # 何も追加していなければGitにアクセスしない
        print("No added submissions, end process")
    else:
        # GitHubにプッシュ
        import git
        import datetime

        dt_now = datetime.datetime.now()
        repo = git.Repo()
        repo.git.add("submissions/*")
        repo.git.commit("submissions/*", message="add submission: " + dt_now.strftime('%Y/%m/%d %H:%M:%S'))
        repo.git.push("origin", "main")

        print(f"Finished process, added {add_cnt} files")
