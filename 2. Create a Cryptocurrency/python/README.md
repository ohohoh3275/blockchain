### 실행 순서

1. `python3 -m venv 01`

2. `source 01/bin/activate`

3. `pip3 install flask`
   (`set FLASK_APP=main.py && set FLASK_ENV=development && FLASK_RUN_PORT=5000`)

   - automatic script needed

4. `pip3 install requests==2.18.4`

5. `export FLASK_APP=main.py && flask run`
