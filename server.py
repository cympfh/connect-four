import re
import subprocess

from fastapi import FastAPI
from fastapi.responses import HTMLResponse

app = FastAPI()
game_pattern = re.compile(r"^[ox\.;]*$")


@app.get("/", response_class=HTMLResponse)
async def index():
    return """
<html>
    <head>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/bulma/0.4.3/css/bulma.min.css">
        <script defer src="https://use.fontawesome.com/releases/v5.0.6/js/all.js"></script>
<style>
div.line:hover {
    cursor: pointer;
    background-color: #f0f0f0;
}
div.column.x {
    background-color: #dd8888;
}
div.column.x.new {
    background-color: #dd5555;
}
div.column.o {
    background-color: #88dd88;
}
div.column.o.new {
    background-color: #55dd55;
}
div#section_game {
    display: none;
}
div#section_finished {
    display: none;
}
</style>
    </head>
    <body>
        <div class="section">
            <nav class="navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="title">Connect-Four 6x7</h1>
                </div>
            </nav>
        </div>

        <div class="section" id="section_choose">
            <div class="container">
                <h1 class="title">Choose</h1>
                <a class="button" id="button_start_human">You are first</a>
                <a class="button" id="button_start_cpu">CPU are first</a>
            </div>
        </div>
        <div class="section" id="section_finished">
            <div class="container">
                <h1 class="title" id="result">Game Finished</h1>
                <p><i class="fas fa-redo"></i> <a href="javascript:location.reload()">Page Reload to Restart</a></p>
            </div>
        </div>
        <div class="section" id="section_game">
            <div class="container">
                <div class="section" style="max-width: 30rem !important">
                    <div class="container">
                        <div id="game"></div>
                    </div>
                </div>
                <div class="section">
                    <p>If you cannot see 6x7 table, You need more wide monitor. Mobile Phone...??</p>
                </div>
            </div>
        </div>

        <div class="section">
            <div class="container">
                <h1 class="title">Rule?</h1>
                <p><a href="https://ja.wikipedia.org/wiki/%E5%9B%9B%E7%9B%AE%E4%B8%A6%E3%81%B9">四目並べ</a></p>
            </div>
        </div>

        <script>
            // events
            document.getElementById('button_start_human').addEventListener('click', (e) => init('o'));
            document.getElementById('button_start_cpu').addEventListener('click', (e) => init('x'));

            var nextplayer = 'x';

            function init(next) {
                nextplayer = next;
                var table = document.getElementById("game");
                table.innerHTML = '';
                table.classList.add('columns');
                table.classList.add('is-vcentered');
                for (var j = 0; j < 7; ++j) {
                    var line_parent = document.createElement('div');
                    line_parent.classList.add('column');
                    var line_child = document.createElement('div');
                    line_child.classList.add('columns');
                    line_child.classList.add('is-vcentered');
                    line_child.classList.add('is-multiline');
                    line_child.classList.add('line');
                    for (var i = 0; i < 6; ++i) {
                        var td = document.createElement('div');
                        td.classList.add('column');
                        td.classList.add('is-full');
                        td.classList.add('empty');
                        td.id = `td_${i}_${j}`;
                        td.innerHTML = `${i},${j}`;
                        line_child.appendChild(td);
                        td.addEventListener('click', (j => ((e) => choice(j)))(j));
                    }
                    line_parent.appendChild(line_child);
                    table.appendChild(line_parent);
                }
                if (next == 'x') { // CPU
                    solve();
                }

                document.getElementById('section_choose').style.display = 'none';
                document.getElementById('section_game').style.display = 'block';
            }

            function finishjudge() {
                var xhr = new XMLHttpRequest();
                xhr.addEventListener("load", () => {
                    var response = JSON.parse(xhr.responseText);
                    if (response.stdout) {
                        if (response.stdout == "X Win" || response.stdout == "O Win" || response.stdout == "Draw") {
                            var msg = response.stdout == "X Win" ? "CPU Win" :
                            response.stdout == "O Win" ? "You Win" : "Draw";
                            document.getElementById('result').innerHTML = msg;
                            document.getElementById('section_finished').style.display = 'block';
                            nextplayer = '-';
                        }
                    }
                });
                xhr.open("GET", `/solve/${game_to_code()}/o`, true);
                xhr.send();
            }

            function solve() {
                var xhr = new XMLHttpRequest();
                xhr.addEventListener("load", () => {
                    var response = JSON.parse(xhr.responseText);
                    if (response.stdout) {
                        update(response.stdout.split("\\n"));
                        finishjudge();
                        nextplayer = 'o';
                    } else {
                        console.log(response);
                    }
                });
                xhr.open("GET", `/solve/${game_to_code()}/x`, true);
                xhr.send();
            }

            function update(lines) {
                for (var i = 0; i < 6; ++i) {
                    for (var j = 0; j < 7; ++j) {
                        var td = document.getElementById(`td_${i}_${j}`);
                        if (lines[i][j] == 'x' && td.classList.contains('empty')) {
                            td.classList.remove('empty');
                            td.classList.add('x');
                            td.classList.add('new');
                        } else if (lines[i][j] == 'o' && td.classList.contains('empty')) {
                            td.classList.remove('empty');
                            td.classList.add('o');
                            td.classList.add('new');
                        }
                    }
                }
            }

            function game_to_code() {
                var code = [];
                for (var i = 0; i < 6; ++i) {
                    var line = '';
                    for (var j = 0; j < 7; ++j) {
                        var td = document.getElementById(`td_${i}_${j}`);
                        line += td.classList.contains('x') ? 'x' : td.classList.contains('o') ? 'o' : '.';
                    }
                    code.push(line);
                }
                return code.join(';');
            }

            function remove_new() {
                for (var i = 0; i < 6; ++i) {
                    for (var j = 0; j < 7; ++j) {
                        var td = document.getElementById(`td_${i}_${j}`);
                        td.classList.remove('new');
                    }
                }
            }

            function choice(j) {
                if (nextplayer != 'o') return;
                for (var i = 5; i >= 0; --i) {
                    var td = document.getElementById(`td_${i}_${j}`);
                    if (td.classList.contains('empty')) {
                        remove_new();
                        td.classList.remove('empty');
                        td.classList.add('o');
                        td.classList.add('new');
                        nextplayer = 'x';
                        finishjudge();
                        solve();
                        return;
                    }
                }
            }

        </script>
    </body>
</html>
    """


@app.get("/solve/{game}/{next_player}")
async def solve(game, next_player):

    assert game_pattern.match(game)
    assert next_player == "o" or next_player == "x"

    result = subprocess.run(
        f"echo '{game}' | tr ';' '\n'| cargo run --release -- --next {next_player}",
        shell=True,
        capture_output=True,
    )
    print(result)
    return {
        "stdout": result.stdout.strip(),
        "stderr": result.stderr.strip(),
    }
