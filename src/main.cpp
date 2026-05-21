#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <conio.h>
#include <windows.h>
#include <cstdio>
using namespace std;

// ================================================================
//  Terminal Text Editor v2.0 - Windows Compatible
// ================================================================

class TextEditor
{
private:
   vector<string> lines;
   int cursorRow = 0;
   int cursorCol = 0;
   string filename = "untitled.txt";

   void setCursorPosition(int row, int col)
   {
      COORD coord;
      coord.X = col;
      coord.Y = row;
      SetConsoleCursorPosition(GetStdHandle(STD_OUTPUT_HANDLE), coord);
   }

public:
   TextEditor()
   {
      lines.push_back("");
   }

   void clearScreen()
   {
      system("cls");
   }

   void display()
   {
      clearScreen();

      // Display all lines with line numbers
      for (int i = 0; i < lines.size(); i++)
      {
         setCursorPosition(i, 0);
         printf("%3d | ", i + 1);
         cout << lines[i];
      }

      // Status bar at bottom
      setCursorPosition(24, 0);
      cout << "--------------------------------------------------------";
      setCursorPosition(25, 0);
      cout << " " << filename << " | Ctrl+S: Save | Ctrl+Q: Quit";

      // Move cursor to edit position
      setCursorPosition(cursorRow, cursorCol + 6);
   }

   void refreshCursor()
   {
      setCursorPosition(cursorRow, cursorCol + 6);
   }

   void insertChar(char ch)
   {
      if (cursorRow >= lines.size())
      {
         lines.push_back("");
         cursorRow = lines.size() - 1;
      }

      lines[cursorRow].insert(cursorCol, 1, ch);
      cursorCol++;
   }

   void deleteChar()
   {
      if (cursorCol > 0)
      {
         lines[cursorRow].erase(cursorCol - 1, 1);
         cursorCol--;
      }
      else if (cursorRow > 0)
      {
         cursorCol = lines[cursorRow - 1].length();
         lines[cursorRow - 1] += lines[cursorRow];
         lines.erase(lines.begin() + cursorRow);
         cursorRow--;
      }
   }

   void newLine()
   {
      string rightPart = lines[cursorRow].substr(cursorCol);
      lines[cursorRow] = lines[cursorRow].substr(0, cursorCol);
      lines.insert(lines.begin() + cursorRow + 1, rightPart);
      cursorRow++;
      cursorCol = 0;
   }

   void moveUp()
   {
      if (cursorRow > 0)
      {
         cursorRow--;
         if (cursorCol > lines[cursorRow].length())
         {
            cursorCol = lines[cursorRow].length();
         }
      }
   }

   void moveDown()
   {
      if (cursorRow < lines.size() - 1)
      {
         cursorRow++;
         if (cursorCol > lines[cursorRow].length())
         {
            cursorCol = lines[cursorRow].length();
         }
      }
   }

   void moveLeft()
   {
      if (cursorCol > 0)
      {
         cursorCol--;
      }
      else if (cursorRow > 0)
      {
         cursorRow--;
         cursorCol = lines[cursorRow].length();
      }
   }

   void moveRight()
   {
      if (cursorCol < lines[cursorRow].length())
      {
         cursorCol++;
      }
      else if (cursorRow < lines.size() - 1)
      {
         cursorRow++;
         cursorCol = 0;
      }
   }

   void save()
   {
      ofstream file(filename);
      for (int i = 0; i < lines.size(); i++)
      {
         file << lines[i];
         if (i < lines.size() - 1)
            file << "\n";
      }
      file.close();
   }
};

int main()
{
   TextEditor editor;
   editor.display();

   while (true)
   {
      if (_kbhit())
      {
         int ch = _getch();

         // Ctrl+Q - Quit
         if (ch == 17)
            break;

         // Ctrl+S - Save
         if (ch == 19)
         {
            editor.save();
            editor.display();
            continue;
         }

         // Enter
         if (ch == 13)
         {
            editor.newLine();
         }
         // Backspace
         else if (ch == 8)
         {
            editor.deleteChar();
         }
         else if (ch == 0 || ch == 224)
         {
            ch = _getch();
            if (ch == 72)
               editor.moveUp();
            else if (ch == 80)
               editor.moveDown();
            else if (ch == 75)
               editor.moveLeft();
            else if (ch == 77)
               editor.moveRight();

            editor.refreshCursor(); 
            continue;               // Skip full display
         }
         // Normal character
         else if (ch >= 32 && ch <= 126)
         {
            editor.insertChar((char)ch);
         }

         editor.display();
      }
   }

   editor.clearScreen();
   cout << "Editor closed. File saved as 'untitled.txt'\n";
   return 0;
}
