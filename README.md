# mama
**A command line application for managing [todo.txt](http://todotxt.org/)**

## Installation
**With cargo**
```console
$ cargo install mama
```
**From git source**
```console
$ cargo install --git https://github.com/teervo/mama.git
```

## Usage
```console
A command line application for managing todo.txt

Usage: mama <command> [arguments]

Available commands:
add           Add a new task to the list
complete      Mark a task as completed
help          Show help for a command
ls            List all tasks
rm            Remove a task from the list
uncomplete    Mark a previously finished task as uncompleted
undo          Undo previous command

```

## Examples
**List all tasks**
```console
$ mama
  ID
   1 Implement new color scheme for @kitchen appliances +kitchenmakeover
   2 Renew subscription to Hobby Horse Monthly
   3 Replenish the pisco decanters in the @office
```

**List tasks matching words**
```console
$ mama ls kitchen office
  ID
   1 Implement new color scheme for @kitchen appliances +kitchenmakeover
   3 Replenish the pisco decanters in the @office
```

**Long listing includes creation date, completion date and priority**
```console
$ mama ls -l
  ID Pri Completed  Created
   1                2021-06-09 Implement new color scheme for @kitchen appliances +kitchenmakeover
   2  C             2021-06-02 Renew subscription to Hobby Horse Monthly
   3  A             2021-06-13 Replenish the pisco decanters in the @office
```

**Add a new task**
```console
$ mama add Alphabetize spice rack @kitchen +kitchenmakeover
+ Adding 'Alphabetize spice rack @kitchen +kitchenmakeover' to todo.txt...

  ID
   1 Implement new color scheme for @kitchen appliances +kitchenmakeover
   2 Renew subscription to Hobby Horse Monthly
   3 Replenish the pisco decanters in the @office
   4 Alphabetize spice rack @kitchen +kitchenmakeover
```

**Add a new task with priority level A**
```console
$ mama add -p A Procure ingredients for the Odelmaß
+ Adding 'Procure ingredients for the Odelmaß' to todo.txt...

  ID Pri Completed  Created
   1                2021-06-09 Implement new color scheme for @kitchen appliances +kitchenmakeover
   2  C             2021-06-02 Renew subscription to Hobby Horse Monthly
   3  A             2021-06-13 Replenish the pisco decanters in the @office
   4                2021-06-16 Alphabetize spice rack @kitchen +kitchenmakeover
   5  A             2021-06-16 Procure ingredients for the Odelmaß
```

**Mark a task as completed**
```console
$ mama complete 3
✅ Completed task 3, 'Replenish the pisco decanters in the @office'

  ID
   1 Implement new color scheme for @kitchen appliances +kitchenmakeover
   2 Renew subscription to Hobby Horse Monthly
✔  3 Replenish the pisco decanters in the @office
   4 Alphabetize spice rack @kitchen +kitchenmakeover
   5 Procure ingredients for the Odelmaß
```

**Remove one or more tasks**
```console
$ mama add rm 2 3
❌ Deleted task 2, 'Renew subscription to Hobby Horse Monthly'.
❌ Deleted task 3, 'Replenish the pisco decanters in the @office'.

  ID
   1 Implement new color scheme for @kitchen appliances +kitchenmakeover
   2 Alphabetize spice rack @kitchen +kitchenmakeover
   3 Procure ingredients for the Odelmaß
```

