def getSubmissionData():
    import requests
    import json

    conf_dir = "conf.json"
    with open(conf_dir, mode="rt", encoding="utf-8") as f:
        config = json.load(f)

    userID = config["userID"]
    unix_second = 0
    api_path = "https://kenkoooo.com/atcoder/atcoder-api/v3/user/submissions?user=" + userID + "&from_second=" + str(unix_second)
    api_url = api_path
    response = requests.get(api_url)
    jsonData = response.json()
    return jsonData
