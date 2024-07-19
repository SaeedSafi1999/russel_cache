using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace CacheUI.Models.Shared
{
    public class ApiResponse<T>
    {
        public string Status { get; set; }
        public T Data { get; set; }
    }
}
