

def get_task(task: int) -> str:
    with open(f"tasks/task{task:02}.txt", "r") as f:
        return f.read()