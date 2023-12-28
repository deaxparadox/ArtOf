class Solution(object):
    def longestCommonPrefix(self, strs: list[str]):
        """
        :type strs: List[str]
        :rtype: str
        """
        # strs.sort(key=lambda k: len(k))
        # match len(strs):
        #     case 0:
        #         return ""
        #     case 1:
        #         return strs[0]
        #     case _:
        #         c = ''
        #         ans = []
        #         i = 0
        #         h = True
        #         for k in range(1, len(strs)):
        #             if not h:
        #                 break
        #             if strs[k] == "":
        #                 continue
        #             if i < len(strs[0]):
        #                 c = strs[0][i]
        #             # else:
                        
        #             if c in strs[k]:
        #                 h = True
        #             else: 
        #                 h = False
        #                 continue
        #             ans.append(c)
        #             i+=1
        #         return "".join(ans)


        
        strs.sort(key=lambda k: len(k))
        match len(strs):
            case 0:
                return ""
            case 1:
                return strs[0]
            case _:
                i = 0
                j = 0
                ans = ''
                breaker = True
                while breaker:
                    if j >=  len(strs[0]):
                        break
                    for i in range(1, len(strs)):
                        # print(i, j, strs[i], strs[i][j], strs[0], strs[0][j])
                        if strs[i][j] != strs[0][j]: 
                            breaker = False
                            break
                    if not breaker:
                        continue
                    ans += strs[0][j]
                    j+=1
                return ans
                    


def main():
    strs = [
        ['a'],
        ["","b"],
        ["flower","flow","flight"],
        ["dog","racecar","car"]
    ]
    
    s = Solution()
    for st in strs:
        print(s.longestCommonPrefix(st))
    
if __name__ == "__main__":
    main()