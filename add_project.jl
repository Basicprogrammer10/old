function input(prompt::String)
    print(prompt)
    return readline()
end

git_address = input("[ GIT URL ] ")
branch = input("[ BRANCH ] ")
folder_name = input("[ FOLDER NAME ] ")

run(`git subtree add $git_address $branch --prefix $folder_name`)