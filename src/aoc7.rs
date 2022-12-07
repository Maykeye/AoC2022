use crate::utils;
const AOC : u32 = 7;


#[derive(Clone, Copy)]
struct INodeIdx(usize);
#[derive(Clone, Copy)]
struct DirIdx(usize);
#[derive(Clone, Copy)]
struct FileIdx(usize);


struct Directory
{
    inode : INodeIdx,
    name : String,
    children : Vec<INodeIdx>,
    own_size: usize,
    total_size: usize,
    parent: Option<INodeIdx>
}

struct File
{
    name: String,
    size: usize
}

enum INode
{
    Directory(DirIdx),
    File(FileIdx)
}

struct Filesystem
{
    inodes : Vec<INode>,
    directories: Vec<Directory>,
    files : Vec<File>
}

impl Filesystem {
    fn new() -> Self {
        let root = Directory {
            inode : INodeIdx(0),
            name: "/".to_string(),
            children: vec![],
            parent: None,
            own_size: 0,
            total_size: 0
        };
        let root_inode = INode::Directory(DirIdx(0));
        Self {
            inodes: vec![root_inode],
            directories: vec![root],
            files: vec![]
        }
    }

    fn calc_own_size(&mut self) {
        for i in 0..self.directories.len() {
            self.directories[i].own_size = self.directories[i]
                .children
                .iter()
                .flat_map(|inode| self.get_file_of_inode(*inode))
                .map(|x| x.size).sum();
        }
    }

    fn impl_propogate_total_size(&mut self, pwd: DirIdx, to_add: usize)
    {
        self.directories[pwd.0].total_size += to_add;
        if let Some(parent) = self.directories[pwd.0].parent {
            let parent = match self.inodes[parent.0] {
                INode::Directory(p) => p,
                _ => unreachable!()
            };
            self.impl_propogate_total_size(parent, to_add);
        }
    }

    fn calc_total_size(&mut self){
        for i in 0..self.directories.len() {
            self.impl_propogate_total_size(DirIdx(i), self.directories[i].own_size);
        }
    }


    fn get_inode(&self, inode: INodeIdx)->&INode{
        &self.inodes[inode.0]
    }
    fn get_file(&self, file:FileIdx) -> &File{
        &self.files[file.0]
    }
    fn get_dir(&self, dir:DirIdx) -> &Directory{
        &self.directories[dir.0]
    }
    fn mut_dir(&mut self, dir:DirIdx) -> &mut Directory{
        &mut self.directories[dir.0]
    }
    fn get_file_of_inode(&self, inode:INodeIdx) -> Option<&File>{
        if let INode::File(fil) = self.get_inode(inode){
            Some(self.get_file(*fil))
        } else {
            None
        }
    }

    fn get_dir_of_inode(&self, inode:INodeIdx) -> Option<&Directory>{
        if let INode::Directory(dir) = self.get_inode(inode){
            Some(self.get_dir(*dir))
        } else {
            None
        }
    }

    fn mut_dir_of_inode(&mut self, inode:INodeIdx) -> Option<&mut Directory>{
        if let INode::Directory(dir) = self.get_inode(inode){
            Some(self.mut_dir(*dir))
        } else {
            None
        }
    }


    /// Allocate new directory inode
    fn impl_mkdir_new(&mut self,
                      parent: INodeIdx,
                      name: &str
    ) -> INodeIdx {
        let new_inode = INodeIdx(self.inodes.len());
        let directory = Directory{
            inode : new_inode,
            name : name.to_string(),
            children : vec![],
            total_size: 0,
            own_size: 0,
            parent: Some(parent)
        };

        let dir_size = self.directories.len();

        self.inodes.push(INode::Directory(DirIdx(dir_size)));

        self.directories.push(directory);

        let dir = self.mut_dir_of_inode(parent).unwrap();
        dir.children.push(new_inode);

        new_inode
    }

    fn mkdir_maybe(&mut self, parent: INodeIdx, name: &str) -> INodeIdx
    {
        let parent = self.get_dir_of_inode(parent).unwrap();
        let maybe_existing_child = parent
            .children
            .iter()
            .flat_map(|x| self.get_dir_of_inode(*x))
            .find(|&x| x.name == name)
            ;

        if let Some(existing_child) = maybe_existing_child {
            return existing_child.inode;
        }

        self.impl_mkdir_new(parent.inode, name)
    }


    /// Allocate new directory inode
    fn mkfile(&mut self, parent: INodeIdx, name: &str, size: usize) -> INodeIdx {
        let new_inode = INodeIdx(self.inodes.len());
        let file = File{
            name: name.to_string(),
            size,
        };

        let files_size = self.files.len();

        self.inodes.push(INode::File(FileIdx(files_size)));

        self.files.push(file);

        let dir = self.mut_dir_of_inode(parent).unwrap();
        dir.children.push(new_inode);

        new_inode
    }

    fn impl_print(&self, pwd: INodeIdx, pfx: &mut String)
    {
        let dir = self.get_dir_of_inode(pwd).unwrap();
        println!("{} {}", *pfx, &dir.name);
        pfx.push('`');
        for child in dir.children.iter() {
            if self.get_dir_of_inode(*child).is_some() {
                self.impl_print(*child, pfx)
            } else {
                let fil = self.get_file_of_inode(*child).unwrap();
                println!("{} {} {}", *pfx, &fil.name, fil.size);
            }
        }
        pfx.pop();
    }

    fn print(&self)
    {
        let mut prefix = String::new();
        self.impl_print(INodeIdx(0), &mut prefix)
    }


}



fn input() -> Filesystem {
    let lines = utils::input_file_lines(AOC);
    let mut fs = Filesystem::new();

    let mut pwd = INodeIdx(0);
    let mut pwd_stack = vec![];

    for line in lines.iter() {
        // We skip cd /, as it happens once per input at lines[0]
        // and we initialized everything already
        if line == &"$ cd /"{
            continue;
        }
        // We skip $ ls as it's the only command that can produce output
        // anyway
        if line == &"$ ls" {
            continue;
        }
        // CD.. goes up
        if line == &"$ cd .."{
            pwd = pwd_stack.pop().unwrap();
            continue;
        }
        // CD.. goes downwards
        let cd_prefix = "$ cd ";
        if line.starts_with(&cd_prefix){
            // save pwd for later
            pwd_stack.push(pwd);
            pwd = fs.mkdir_maybe(pwd, &line[cd_prefix.len()..]);

            continue;
        }

        let mut tokens = line.split_ascii_whitespace();
        let lhs = tokens.next().unwrap();
        let name = tokens.next().unwrap();
        if lhs == "dir" {
            fs.mkdir_maybe(pwd, name);
        } else {
            let size = lhs.parse().unwrap();
            fs.mkfile(pwd, name, size);
        }
    }

    fs

}

fn part1()
{
    let mut fs = input();

    fs.calc_own_size();
    fs.calc_total_size();
    fs.print();

    let p1 : usize = fs.directories
        .iter()
        .filter(|d| d.total_size <= 100000)
        .map(|d| d.total_size)
        .sum();

    dbg!(p1);
}
fn part2()
{
    let mut fs = input();

    fs.calc_own_size();
    fs.calc_total_size();

    let total_size = 70000000;
    let used_size = fs.directories[0].total_size;
    let free_size = total_size - used_size;

    let required_size_for_update = 30000000;
    let size_to_be_freed = required_size_for_update - free_size;

    let size_deleted : usize =  fs.directories.iter()
        .filter(|dir| dir.total_size >= size_to_be_freed)
        .map(|dir| dir.total_size)
        .min().unwrap();






    dbg!(size_deleted);
}


pub fn run()
{
    if utils::is_part_1() {
        part1();
    } else {
        part2();
    }
}
