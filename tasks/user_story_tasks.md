# To-Do App User Story Tasks

## 1. As a user, I want to create a new task so that I can remember what I need to do.

1. Write test for Task struct creation
2. Implement Task struct
3. Write test for TaskRepository::create method
4. Implement TaskRepository::create method
5. Write test for create_task handler
6. Implement create_task handler
7. Write integration test for POST /tasks endpoint
8. Implement POST /tasks endpoint

## 2. As a user, I want to view all my tasks so that I can see what I need to work on.

1. Write test for TaskRepository::get_all method
2. Implement TaskRepository::get_all method
3. Write test for get_all_tasks handler
4. Implement get_all_tasks handler
5. Write integration test for GET /tasks endpoint
6. Implement GET /tasks endpoint

## 3. As a user, I want to mark a task as complete so that I can track my progress.

1. Write test for Task::mark_as_complete method
2. Implement Task::mark_as_complete method
3. Write test for TaskRepository::update method
4. Implement TaskRepository::update method
5. Write test for mark_task_complete handler
6. Implement mark_task_complete handler
7. Write integration test for PATCH /tasks/{id}/complete endpoint
8. Implement PATCH /tasks/{id}/complete endpoint

## 4. As a user, I want to set due dates for tasks so that I can prioritize my work.

1. Write test for Task::set_due_date method
2. Implement Task::set_due_date method
3. Write test for TaskRepository::update_due_date method
4. Implement TaskRepository::update_due_date method
5. Write test for set_task_due_date handler
6. Implement set_task_due_date handler
7. Write integration test for PATCH /tasks/{id}/due-date endpoint
8. Implement PATCH /tasks/{id}/due-date endpoint

## 5. As a user, I want to edit task details so that I can update information as needed.

1. Write test for Task::update_details method
2. Implement Task::update_details method
3. Write test for TaskRepository::update_details method
4. Implement TaskRepository::update_details method
5. Write test for update_task_details handler
6. Implement update_task_details handler
7. Write integration test for PUT /tasks/{id} endpoint
8. Implement PUT /tasks/{id} endpoint

## 6. As a user, I want to delete tasks so that I can remove items that are no longer relevant.

1. Write test for TaskRepository::delete method
2. Implement TaskRepository::delete method
3. Write test for delete_task handler
4. Implement delete_task handler
5. Write integration test for DELETE /tasks/{id} endpoint
6. Implement DELETE /tasks/{id} endpoint

## 7. As a user, I want to categorize my tasks so that I can organize them better.

1. Write test for Category struct creation
2. Implement Category struct
3. Write test for Task::set_category method
4. Implement Task::set_category method
5. Write test for CategoryRepository::create method
6. Implement CategoryRepository::create method
7. Write test for TaskRepository::update_category method
8. Implement TaskRepository::update_category method
9. Write test for create_category handler
10. Implement create_category handler
11. Write test for set_task_category handler
12. Implement set_task_category handler
13. Write integration test for POST /categories endpoint
14. Implement POST /categories endpoint
15. Write integration test for PATCH /tasks/{id}/category endpoint
16. Implement PATCH /tasks/{id}/category endpoint

## 8. As a user, I want to receive reminders for upcoming due dates so that I don't miss deadlines.

1. Write test for Task::is_due_soon method
2. Implement Task::is_due_soon method
3. Write test for TaskRepository::get_tasks_due_soon method
4. Implement TaskRepository::get_tasks_due_soon method
5. Write test for ReminderService::send_reminder method
6. Implement ReminderService::send_reminder method
7. Write test for check_and_send_reminders job
8. Implement check_and_send_reminders job
9. Write integration test for reminder system
10. Set up cron job or scheduled task for running reminders