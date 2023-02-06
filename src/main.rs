use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct CreateOptions {
	        #[structopt(short, long, default_value="dgcgced", help="Set avalue\n")]
		avalue: String,

		#[structopt(short, long, default_value="dehwuio", help="Set bvalue\n")]
		bvalue: String,
		
		#[structopt(short, long, default_value="bvsdrwe", help="Set cvalue\n")]
		cvalue: String,			
		
		#[structopt(short, long, default_value="utrwegf", help="Set dvalue\n")]
		dvalue: String,
					
		#[structopt(short, long, default_value="xbcfuib", help="Set evalue. Value: \
											    \n		aaa:	cvcii \
											    \n		bbb:	fuhov \n")]    
		evalue: String,
		
		#[structopt(short, long, default_value="xvweuov", help="Set fvalue\n")]    
		fvalue: String,
		
		#[structopt(short, long, default_value="bxwcdgd", help="Set gvalue. Values: \
											    \n		0:	uigci \
											    \n		1:	dfrrv \
											    \n		2:	grtyh \
											    \n		3:	bggsd \
											    \n		4:	eydgg \n")]
		gvalue: String,
}

#[derive(StructOpt, Debug)]
pub enum CmdCommand {
    #[structopt()]
    Aa,
    Bb(CreateOptions),
    Cc,
    Dd,
    Ee,
    Ff,
    Gg,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Cmd(CmdCommand),
}

#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(subcommand)]
    pub command: Command,
}

fn main() {
    let options: Options = Options::from_args();
    match options.command {
        Command::Cmd(prj_command) => {
            match prj_command {
                CmdCommand::Aa     => println!("aa ..."),
		
		
		
		
                CmdCommand::Bb(create_options) => { println!("bb ...");
		 					println!("pulling items :\n \t{}\n \t{}\n \t{}\n \t{}\n \t{}\n \t{}\n \t{}\n ",
		 								create_options.avalue,
		 								create_options.bvalue,
		 								create_options.cvalue,
		 								create_options.dvalue,
		 								create_options.evalue,
		 								create_options.fvalue,
		 								create_options.gvalue,
		 								);
		 				      },
                CmdCommand::Cc => println!("cc ..."),
                CmdCommand::Dd => println!("dd ..."),
                CmdCommand::Ee => println!("ee ..."),
                CmdCommand::Ff => println!("ff ..."),
                CmdCommand::Gg => println!("gg ..."),
            }
        }
    }
   
}
