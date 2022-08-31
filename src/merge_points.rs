use crate::corner_detector::Coordinate;

fn euclidean_distance(p1x: usize, p1y: usize, p2x: usize, p2y: usize) -> usize { //点A,Bのユークリッド距離を求める(p1x,p1y...点AのX,Y座標,  p2x,p2y...点BのX,Y座標)
    let dx: usize;
    let dy: usize;
    let d2: usize;
    let mut d1: usize = 1;
    if p1x > p2x {
        dx = p1x - p2x;
    } else {
        dx = p2x - p1x;
    }
    if p1y > p2y {
        dy = p1y - p2y;
    } else {
        dy = p2y - p1y;
    }
    d2 = dx * dx + dy * dy;
    while d1 * d1 < d2 {
        d1 += 1;
    }
    return d1;
}

fn merge_points(points: Vec<Coordinate>, adjacent: Vec<Vec<usize>>, n: usize) -> (Vec<Coordinate>, Vec<Vec<usize>>) { // 点を結合する(このプログラムのメイン)
    let mut m1: Vec<Coordinate> = Vec::new();//出力する点の座標リスト
    let mut m2: Vec<Coordinate> = Vec::new();//メモする点の座標リスト
    let mut a1: Vec<Vec<usize>> = adjacent.clone();//出力する点の隣接行列
    let mut a2: Vec<Vec<usize>> = Vec::new();//メモする点の隣接行列
    let mut ai: Vec<usize> = Vec::new();//隣接行列の第i行のメモ
    let mut i: usize;//第何行？
    let mut j: usize;//第何列？
    let mut count: usize = 0;//pointsやadjacentを削るのは何回目？
    let mut labela: usize = 0;//結合点候補A
    let mut labelb: usize = 0;//結合点候補B
    let mut min: usize = 0;//結合する点の間の距離
    let mut pl: usize = points.len();
    for i in 0..pl {
        m1.push(Coordinate { x: (points[i].x), y: (points[i].y)});
    }
    while pl - count > n {
        min = 0;
        labela = 0;
        labela = 0;
        for i in 0..pl - count {
            for j in i..pl - count{
                if a1[i][j] != 0 && i < j{
                    if min == 0 || min > a1[i][j] {
                        min = a1[i][j];
                        labela = i;
                        labelb = j;
                    }
                }
            }
        }
        //println!("点{}と点{}を結合します",labela + 1, labelb + 1);
        m2 = Vec::new();
        for i in 0..pl - count {
            m2.push(Coordinate { x: (m1[i].x), y: (m1[i].y)});
        }
        m1 = Vec::new();
        for i in 0..labela {
            m1.push(Coordinate { x: (m2[i].x), y: (m2[i].y)});
        }
        m1.push(Coordinate { x: ((m2[labela].x + m2[labelb].x) / 2), y: ((m2[labela].y + m2[labelb].y) / 2)});
        for i in labelb + 1..pl - count {
            m1.push(Coordinate { x: (m2[i].x), y: (m2[i].y)});
        }
        a2 = a1.clone();
        a1 = Vec::new();
        for i in 1..labela + 1 {
            ai = Vec::new();
            for j in 1..labela + 1{
                if a2[i - 1][j - 1] > 0{
                    ai.push(a2[i - 1][j - 1]);
                } else {
                    ai.push(0);
                }
            }
            if a2[i - 1][labela] != 0 || a2[i - 1][labelb] != 0 {
                ai.push(euclidean_distance(m1[labela].x, m1[labela].y, m1[i - 1].x, m1[i - 1].y));
            } else {
                ai.push(0);
            }
            for j in labelb + 1..pl - count {
                if a2[i - 1][j] > 0 {
                    ai.push(a2[i - 1][j]);
                } else {
                    ai.push(0);
                }
            }
            a1.push(ai.clone());
        }
        ai = Vec::new();
        for j in 0..labela{
            if a2[labela][j] != 0 || a2[labelb][j] != 0 {
                ai.push(euclidean_distance(m1[labela].x, m1[labela].y, m1[j].x, m1[j].y));
            } else {
                ai.push(0);
            }
        }
        ai.push(0);//そりゃ、隣接行列の対角成分は0だもの。
        for j in labelb + 1..pl - count {
            if a2[labela][j] != 0 || a2[labelb][j] != 0 {
                ai.push(euclidean_distance(m1[labela].x, m1[labela].y, m1[j - 1].x, m1[j - 1].y));
            } else {
                ai.push(0);
            }
        }
        a1.push(ai.clone());
        for i in labelb..pl - count - 1 {
            ai = Vec::new();
            for j in 1..labela + 1 {
                if a2[i + 1][j - 1] > 0{
                    ai.push(a2[i + 1][j - 1]);
                } else {
                    ai.push(0);
                }
            }
            if a2[i + 1][labela] != 0 || a2[i + 1][labelb] != 0 {
                ai.push(euclidean_distance(m1[labela].x, m1[labela].y, m1[i].x, m1[i].y));
            } else {
                ai.push(0);
            }
            for j in labelb + 1..pl - count{
                if a2[i + 1][j] > 0{
                    ai.push(a2[i + 1][j]);
                } else {
                    ai.push(0);
                }
            }
            a1.push(ai.clone());
        }
        count += 1;
    }
    for i in 0..n {
        for j in 0..n {
            if a1[i][j] != 0 {
                a1[i][j] = 1;
            }
        }
    }
    return (m1, a1);
}
////////////////////////////////////////////////////　　↓↓↓↓↓↓↓動作確認プログラム↓↓↓↓↓↓↓
/* 
fn main() {
    let mut tenn: Vec<Coordinate> = Vec::new();
    let mut rinsetsu: Vec<Vec<usize>> = Vec::new();
    let mut ri: Vec<usize> = Vec::new();
    let d: usize = 4;
    let e: usize;
    let f: usize;
    tenn.push(Coordinate { x: (20), y: (610)});
    tenn.push(Coordinate { x: (15), y: (170)});
    tenn.push(Coordinate { x: (200), y: (400)});
    tenn.push(Coordinate { x: (175), y: (310)});
    tenn.push(Coordinate { x: (160), y: (235)});
    tenn.push(Coordinate { x: (325), y: (710)});
    tenn.push(Coordinate { x: (330), y: (15)});
    ri.push(0);//1
    ri.push(440);
    ri.push(277);
    ri.push(0);
    ri.push(0);
    ri.push(0);
    ri.push(0);
    rinsetsu.push(ri);
    ri = Vec::new();
    ri.push(440);//2
    ri.push(0);
    ri.push(0);
    ri.push(0);
    ri.push(159);
    ri.push(0);
    ri.push(0);
    rinsetsu.push(ri);
    ri = Vec::new();
    ri.push(277);//3
    ri.push(0);
    ri.push(0);
    ri.push(93);
    ri.push(0);
    ri.push(334);
    ri.push(0);
    rinsetsu.push(ri);
    ri = Vec::new();
    ri.push(0);//4
    ri.push(0);
    ri.push(93);
    ri.push(0);
    ri.push(76);
    ri.push(0);
    ri.push(0);
    rinsetsu.push(ri);
    ri = Vec::new();
    ri.push(0);//5
    ri.push(159);
    ri.push(0);
    ri.push(76);
    ri.push(0);
    ri.push(0);
    ri.push(278);
    rinsetsu.push(ri);
    ri = Vec::new();
    ri.push(0);//6
    ri.push(0);
    ri.push(334);
    ri.push(0);
    ri.push(0);
    ri.push(0);
    ri.push(695);
    rinsetsu.push(ri);
    ri = Vec::new();
    ri.push(0);//7
    ri.push(0);
    ri.push(0);
    ri.push(0);
    ri.push(278);
    ri.push(695);
    ri.push(0);
    rinsetsu.push(ri);
    ri = Vec::new();
    e = tenn.len();
    (tenn, rinsetsu) = marge_points(tenn, rinsetsu, d);
    for e in 0..tenn.len() {
        println!("{} {} {}", e, tenn[e].x, tenn[e].y);
    }
    for e in 0..d {
        for f in 0..d {
            print!("{:5}", rinsetsu[e][f]);
        }
        println!("");
    }
}
*/