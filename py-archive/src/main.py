import json
import os
from dataclasses import asdict, dataclass
from enum import StrEnum
from pathlib import Path
from typing import cast

import typer

app = typer.Typer()


class TaskTriage(StrEnum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    URGENT = "urgent"

    def __int__(self) -> int:
        map = {
            TaskTriage.LOW: 0,
            TaskTriage.MEDIUM: 1,
            TaskTriage.HIGH: 2,
            TaskTriage.URGENT: 3,
        }
        return map[self]


class TaskStatus(StrEnum):
    TODO = "todo"
    IN_PROGRESS = "inprog"
    DONE = "done"

    def __int__(self) -> int:
        map = {
            TaskStatus.TODO: 0,
            TaskStatus.IN_PROGRESS: 1,
            TaskStatus.DONE: 2,
        }
        return map[self]


@dataclass
class Task:
    name: str
    triage: TaskTriage
    status: TaskStatus


def get_tasks_file() -> str:
    return f"{os.getenv('XDG_DATA_HOME')}/snowtracks/tasks.json"


def todict_factory(data: list[tuple[str, object]]) -> dict[str, object]:
    res: dict[str, object] = {}
    for k, v in data:
        if isinstance(v, TaskStatus) or isinstance(v, TaskTriage):
            res[k] = int(v)
        else:
            res[k] = v
    return res


@app.command()
def add():
    name: str = input("Enter task name: ")
    triage: TaskTriage
    status: TaskStatus

    while True:
        try:
            triage = TaskTriage(input("Enter triage level: "))
            break
        except ValueError:
            print(
                f"Invalid triage. Valid levels: {', '.join([level.value for level in TaskTriage])}"
            )

    while True:
        try:
            status_str: str = input("Enter task status (todo): ")
            status = TaskStatus.TODO if status_str == "" else TaskStatus(status_str)
            break
        except ValueError:
            print(
                f"Invalid status. Valid levels: {', '.join([level.value for level in TaskStatus])}"
            )

    taskfile: str = get_tasks_file()
    with open(taskfile, encoding="utf-8") as f:
        tasks = cast(list[dict[str, object]], json.load(f))

    new_task: Task = Task(name, triage, status)
    tasks.append(asdict(new_task, dict_factory=todict_factory))

    with open(taskfile, "w", encoding="utf-8") as f:
        json.dump(tasks, f)


@app.command()
def setup():
    xdg_data_home: str | None = os.getenv("XDG_DATA_HOME")
    if not xdg_data_home:
        raise NotADirectoryError("$XDG_DATA_HOME not found or does not exist")
    data_home: Path = Path(xdg_data_home) / "snowtracks"
    db_file: Path = data_home / "tasks.json"

    if db_file.exists() and db_file.stat().st_size != 0:
        return

    print(f"Setting up database in {data_home}...", end=" ")
    data_home.mkdir(parents=True, exist_ok=True)

    with open(f"{db_file}", "w", encoding="utf-8") as f:
        json.dump([], f)

    print("done")


if __name__ == "__main__":
    app()
