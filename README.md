# quiz_late

A customisable quiz program to study another Language's vocabulary.

## Description

This program is designed to be flexible for individuals preferences to creating a **Study Set**, consisting of **Words**.

Each Study Set can have a list of **tags**.

Each **Word** has 3 fields:
1. Word ==> the actual word you would like to learn
2. Word Answer ==> the answer stored to match against your input
3. Remarks ==> the 'hint' or additional info for the word to be shown

## Features

On launch, there will be a list of Study Sets available locally. <br>
(There will not be any item if they do not exist)

If there are tags, there will be a list of buttons for each unique tag.

For all of which, it will change colour on click, to show the toggle state.

You can click on any combination of the tags.

### Filters

You will definitely see a 'Filter by Tags' label with buttons:
#### 1. Match Any
On click, the list will be updated depending on the tag buttons that have been clicked.

The list will contain **ANY** Study Set where there is at least 1 matching tag.

The list will **NOT** be updated if no buttons are selected.

#### 2. Match All
On click, the list will be updated depending on the tag buttons that have been clicked.

The list will contain **ONLY** Study Sets where all the tags matches those selected.

The list will **NOT** be updated if no buttons are selected.

#### 3. See All Sets
On click, the list will be updated to show **ALL** Study Sets available.

#### 4. See All Untagged Sets
On click, the list will be updated to show **ONLY** Study Sets with no tags.

For each of the Study Sets, you can:
---
### 1. View
On clicking `View`, the list of words in the study set is shown in descending order, for each of them you could *Edit* or *Delete*.

You can add new words to the set by `Add Word` button.
    
You can also navigate to previous or next Study Set via buttons at the top.

### 2. Learn
On clicking `Learn`, you will be brought to a page showing 1 word at a time, where you can key in the answer into input field.

You can navigate to previous or next word via buttons at the top.

You can also click `Show Answer` or `Hide Answer` to toggle display answer for the current word.

You can click on `Calculate Score` at any time to see the number of correct/wrong inputs.

### 3. Test
On clicking `Test`, you will be brought to a page showing 1 word at a time, where you can key in the answer into input field. 

This is very similar to `Learn` except there will not be an option to show answer for any of the words.

### 4. Delete
On clicking `Delete`, the Study Set will be deleted and local file will also be removed.

### 5. Edit
On clicking `Edit`, you will be brought to a page where you can rename the study set and see the list of existing tags for the correpsonding study set.

In this page, you can also remove existing tags.

On clicking `Save Changes` the study set and local data file will be renamed as per input (if empty, the file will not be renamed)

Any input for tag field will be added to the set of tags for the study set.


## For All Users

The application is available in [all versions of releases](https://github.com/wanyu-l/quiz_proj/releases).

The application can be run by double clicking the icon, like other desktop applications.

---
### For Windows Users 
The progoram is available [here](https://github.com/wanyu-l/quiz_proj/releases/tag/v2.0.0).

Program is not allowed to execute by windows system by default

#### How to resolve
Navigate as per prompts by windows to allow program to be run.

---
### For Mac Users 
The progoram is available [here](https://github.com/wanyu-l/quiz_proj/releases/tag/v2.0.0-mac).

Program is not allowed to execute by operating system by default

#### How to resolve
1. Open terminal and use `cd <path_to_target_folder>` to move to folder with program.
2. Run `chmod +777 quiz_late` in the folder with program to allow program to be run.
This grants Read/Write/Execute privileges to the program and they are required for program to run, to read from and save to local data files.

