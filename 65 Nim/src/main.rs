use std::io;

fn main() -> io::Result<()> {

    println!("{}{}", " ".repeat(33), "NIM");
    println!("{}{}", " ".repeat(15), "CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY");
    println!();
    println!();
    println!();
    println!("THIS IS THE GAME OF NIM.");
    println!("DO YOU WANT INSTRUCTIONS");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() == "YES" || input.trim() == "yes" {
            println!("THE GAME IS PLAYED WITH A NUMBER OF PILES OF OBJECTS.");
            println!("ANY NUMBER OF OBJECTS ARE REMOVED FROM ONE PILE BY YOU AND");
            println!("THE MACHINE ALTERNATELY.  ON YOUR TURN, YOU MAY TAKE");
            println!("ALL THE OBJECTS THAT REMAIN IN ANY PILE, BUT YOU MUST");
            println!("TAKE AT LEAST ONE OBJECT, AND YOU MAY TAKE OBJECTS FROM");
            println!("ONLY ONE PILE ON A SINGLE TURN.  YOU MUST SPECIFY WHETHER");
            println!("WINNING IS DEFINED AS TAKING OR NOT TAKING THE LAST OBJECT,");
            println!("THE NUMBER OF PILES IN THE GAME, AND HOW MANY OBJECTS ARE");
            println!("ORIGINALLY IN EACH PILE.  EACH PILE MAY CONTAIN A");
            println!("DIFFERENT NUMBER OF OBJECTS.");
            println!("THE MACHINE WILL SHOW ITS MOVE BY LISTING EACH PILE AND THE");
            println!("NUMBER OF OBJECTS REMAINING IN THE PILES AFTER  EACH OF ITS");
            println!("MOVES.");
            break;
        } else if input.trim() == "NO" || input.trim() == "no" {
            break;
        } else {
            println!("PLEASE ANSWER YES OR NO");
        }
    }
    let mut playing = true;
    while playing {
        let mut take_last: bool;
        let mut move_first: bool;
        let mut num_piles: usize;
        let mut pile_sizes: Vec::<i32>;
        println!();
        loop {
            println!("ENTER WIN OPTION - 1 TO TAKE LAST, 2 TO AVOID LAST");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim() == "1" || input.trim() == "2" {
                take_last = input.trim() == "1";
                break;
            }
        }
        loop {
            println!("ENTER NUMBER OF PILES");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if let Ok(n) = input.trim().parse::<usize>() {
                if n > 0 && n < 101 {
                    num_piles = n;
                    break;
                }
            }
        }
        pile_sizes = vec![0, num_piles as i32];
        println!("ENTER PILE SIZES");
        for i in 0..num_piles {
            loop {
                println!("{}", i);
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if let Ok(n) = input.trim().parse::<i32>() {
                    if n > 0 && n < 2001 {
                        pile_sizes[i] = n;
                        break;
                    }
                }
            }
        }
        println!("DO YOU WANT TO MOVE FIRST");
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim() == "YES" || input.trim() == "yes" {
                move_first = true;
                break;
            } else if input.trim() == "NO" || input.trim() == "no" {
                move_first = false;
                break;
            } else {
                println!("PLEASE ANSWER YES OR NO.");
            }
        }
        // the game logic itself
        //TODO

        println!("do you want to play another game");
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim() == "YES" || input.trim() == "yes" {
                break;
            } else if input.trim() == "NO" || input.trim() == "no" {
                playing = false;
                break;
            } else {
                println!("PLEASE.  YES OR NO.");
            }
        }
    }
    Ok(())
}


// 210 DIM A(100),B(100,10),D(2)
// take_last - W == '1'
// num_piles - N
// pile_sizes[i] - A(I)
// move_first - Q9$
//
//
// 620 PRINT "DO YOU WANT TO MOVE FIRST";
// 630 INPUT Q9$
// 640 IF Q9$="YES" THEN 1450
// 650 IF Q9$="yes" THEN 1450
// 660 IF Q9$="NO" THEN 700
// 670 IF Q9$="no" THEN 700
// 680 PRINT "PLEASE ANSWER YES OR NO."
// 690 GOTO 630
//
// 700 IF W=1 THEN 940
// 710 LET C=0
// 720 FOR I=1 TO N
// 730 IF A(I)=0 THEN 770
// 740 LET C=C+1
// 750 IF C=3 THEN 840
// 760 LET D(C)=I
// 770 NEXT I
// 780 IF C=2 THEN 920
// 790 IF A(D(1))>1 THEN 820
// 800 PRINT "MACHINE LOSES"
// 810 GOTO 1640
// 820 PRINT "MACHINE WINS"
// 830 GOTO 1640
// 840 LET C=0
// 850 FOR I=1 TO N
// 860 IF A(I)>1 THEN 940
// 870 IF A(I)=0 THEN 890
// 880 LET C=C+1
// 890 NEXT I
// 900 IF C/2<>INT(C/2) THEN 800
// 910 GOTO 940
// 920 IF A(D(1))=1 THEN 820
// 930 IF A(D(2))=1 THEN 820
// 940 FOR I=1 TO N
// 950 LET E=A(I)
// 960 FOR J=0 TO 10
// 970 LET F=E/2
// 980 LET B(I,J)=2*(F-INT(F))
// 990 LET E=INT(F)
// 1000 NEXT J
// 1010 NEXT I
// 1020 FOR J=10 TO 0 STEP -1
// 1030 LET C=0
// 1040 LET H=0
// 1050 FOR I=1 TO N
// 1060 IF B(I,J)=0 THEN 1110
// 1070 LET C=C+1
// 1080 IF A(I)<=H THEN 1110
// 1090 LET H=A(I)
// 1100 LET G=I
// 1110 NEXT I
// 1120 IF C/2<>INT(C/2) THEN 1190
// 1130 NEXT J
// 1140 LET E=INT(N*RND(1)+1)
// 1150 IF A(E)=0 THEN 1140
// 1160 LET F=INT(A(E)*RND(1)+1)
// 1170 LET A(E)=A(E)-F
// 1180 GOTO 1380
// 1190 LET A(G)=0
// 1200 FOR J=0 TO 10
// 1210 LET B(G,J)=0
// 1220 LET C=0
// 1230 FOR I=1 TO N
// 1240 IF B(I,J)=0 THEN 1260
// 1250 LET C=C+1
// 1260 NEXT I
// 1270 LET A(G)=A(G)+2*(C/2-INT(C/2))*2^J
// 1280 NEXT J
// 1290 IF W=1 THEN 1380
// 1300 LET C=0
// 1310 FOR I=1 TO N
// 1320 IF A(I)>1 THEN 1380
// 1330 IF A(I)=0 THEN 1350
// 1340 LET C=C+1
// 1350 NEXT I
// 1360 IF C/2<>INT(C/2) THEN 1380
// 1370 LET A(G)=1-A(G)
// 1380 PRINT "PILE  SIZE"
// 1390 FOR I=1 TO N
// 1400 PRINT I;A(I)
// 1410 NEXT I
//
// 1420 IF W=2 THEN 1450
// 1430 GOSUB 1570
// 1440 IF Z=1 THEN 820
// 1450 PRINT "YOUR MOVE - PILE, NUMBER TO BE REMOVED";
// 1460 INPUT X,Y
// 1470 IF X>N THEN 1450
// 1480 IF X<1 THEN 1450
// 1490 IF X<>INT(X) THEN 1450
// 1500 IF Y>A(X) THEN 1450
// 1510 IF Y<1 THEN 1450
// 1520 IF Y<>INT(Y) THEN 1450
// 1530 LET A(X)=A(X)-Y
// 1540 GOSUB 1570
// 1550 IF Z=1 THEN 800
// 1560 GOTO 700
// 1570 LET Z=0
// 1580 FOR I=1 TO N
// 1590 IF A(I)=0 THEN 1610
// 1600 RETURN
// 1610 NEXT I
// 1620 LET Z=1
// 1630 RETURN
// 1720 GOTO 440

// 1730 END
