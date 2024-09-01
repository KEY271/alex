import { PieceType } from "../utils/game";

function piecename(pt: PieceType): string {
    switch (pt) {
        case PieceType.None:
            return "";
        case PieceType.Light:
            return "歩";
        case PieceType.Heavy:
            return "重";
        case PieceType.King:
            return "玉";
        case PieceType.Prince:
            return "子";
        case PieceType.General:
            return "将";
        case PieceType.Knight:
            return "騎";
        case PieceType.Arrow:
            return "矢";
        case PieceType.Archer0:
            return "弓";
        case PieceType.Archer1:
            return "弓1";
        case PieceType.Archer2:
            return "弓2";
    }
}

type PieceProps = {
    pt: PieceType;
    demise: number;
};

function Piece(props: PieceProps) {
    const { pt, demise } = props;
    const name = piecename(pt);
    const crown =
        (pt == PieceType.King && demise % 2 == 0) || (pt == PieceType.Prince && demise % 2 == 1) ? (
            <div
                className="absolute left-[-16px] top-[-16px] h-[8px] w-[32px] bg-black
                    [clip-path:polygon(0_100%,0_0,25%_30%,50%_0,75%_30%,100%_0,100%_100%)] sm:left-[-24px]
                    sm:top-[-24px] sm:h-[14px] sm:w-[48px]">
                <div
                    className="h-full w-full bg-yellow-400
                        [clip-path:polygon(2px_calc(100%-2px),2px_3px,25%_calc(30%+2px),50%_2px,75%_calc(30%+2px),calc(100%-2px)_3px,calc(100%-2px)_calc(100%-2px))]"></div>
            </div>
        ) : (
            <></>
        );
    return (
        <div className="relative">
            <div
                className="absolute left-[-16px] top-[-16px] flex h-[32px] w-[32px] items-center justify-center bg-black
                    [clip-path:polygon(50%_0,90%_15%,100%_100%,0_100%,10%_15%)] sm:left-[-24px] sm:top-[-24px]
                    sm:h-[48px] sm:w-[48px]">
                <div
                    className="flex h-[28px] w-[28px] justify-center bg-[burlywood] pt-1 text-base
                        [clip-path:polygon(50%_0,90%_15%,100%_100%,0_100%,10%_15%)] sm:h-[44px] sm:w-[44px] sm:pt-2
                        sm:text-2xl">
                    {name}
                </div>
            </div>
            {crown}
        </div>
    );
}

export default Piece;
